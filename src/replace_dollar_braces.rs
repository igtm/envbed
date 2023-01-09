use crate::model::EnvVar;
use rustc_hash::FxHasher;
use std::{collections::HashMap, hash::BuildHasherDefault, sync::{mpsc, Mutex, Arc}, ops::Range};
use std::thread;

#[allow(dead_code)]
pub fn replace_dollar_braces(envvars: &Vec<EnvVar>, mut target: String) -> String {
    for envvar in envvars {
        target = target.replace(&format!("${{{}}}", &envvar.key), &envvar.val)
    }

    target
}

#[allow(dead_code)]
pub fn replace_dollar_braces_with_hashmap(
    envvars: &HashMap<String, String, BuildHasherDefault<FxHasher>>,
    target: &str,
) -> String {
    let mut result = String::new();
    let mut last_end = 0;
    let mut env_key_start = 0;

    let mut f_match_level: usize = 0; // 0,1=$,2=${,3=${}
    for (i, c) in target.char_indices() {
        if c == '$' {
            result.push_str(unsafe { target.get_unchecked(last_end..i) });
            last_end = i;
            f_match_level = 1;
        } else if c == '{' {
            if f_match_level == 1 {
                f_match_level = 2;
            } else {
                f_match_level = 0;
            }
        } else if c == '_' || c.is_alphanumeric() {
            if f_match_level == 2 {
                f_match_level = 3;
                env_key_start = i;
            }
        } else if c == '}' && f_match_level == 3 {
            // get value from hashmap
            if let Some(val) = envvars.get(unsafe { target.get_unchecked(env_key_start..i) }) {
                result.push_str(val);
                last_end = i + 1;
            }
            env_key_start = 0;
            f_match_level = 0;
        } else {
            env_key_start = 0;
            f_match_level = 0;
        }
    }
    result.push_str(unsafe { target.get_unchecked(last_end..target.len()) });
    result
}


#[allow(dead_code)]
pub fn replace_dollar_braces_with_hashmap_parallel(
    envvars: &HashMap<String, String, BuildHasherDefault<FxHasher>>,
    target: String,
    thread_n: usize,
) -> String {
  replace_parallel(target, envvars, thread_n)
}


pub struct ChannelValue {
  pub a: usize,
  pub partial_match_idx: Option<usize>,
  pub text: String,
  pub replacement: Vec<(Range<usize>, String)>,
}


fn replace_parallel(target: String, envvars: &HashMap<String, String, BuildHasherDefault<FxHasher>>, thread_n: usize) -> String {
  // 非同期チャネルを生成
  let (tx, rx) = mpsc::channel();

  // start
  let mut result = String::new();

  let char_count = target.chars().count();
  let start_end_range_count =
      char_count / thread_n + if char_count % thread_n != 0 { 1 } else { 0 };

  println!("parallel");
  for i in 0..(thread_n) {
      let tx = tx.clone();
      let envvars = envvars.clone();

      // variables
      let start_i = start_end_range_count * i;
      let end_i = if i == (thread_n - 1) {
          char_count
      } else {
          start_end_range_count * (i + 1)
      };
      let target_partial = unsafe { target.get_unchecked(start_i..end_i) }.to_owned();

      thread::spawn(move || {

          // let mut tmp = String::new();
          let mut tmp: Vec<(Range<usize>, String)> = vec![];
          let mut env_key_start = 0;

          let mut f_match_level: usize = 0; // 0,1=$,2=${,3=${}
          let mut partial_match_idx = None;
          let mut idx = 0;
          let mut env_key = String::new();
          for (mut j, c) in target_partial.char_indices().into_iter() {
              j += start_i;
              if end_i <= j {
                  break;
              }
              (f_match_level, env_key_start) = replace(j, c, f_match_level, env_key_start);
              if f_match_level == 1 {
                idx = j;
              } else if f_match_level == 4 {
                if let Some(val) = envvars.get(&env_key) {
                  tmp.push((env_key_start-2..j+1, val.to_owned()));
                  partial_match_idx = None;
                }
                env_key_start = 0;
                f_match_level = 0;
              } else if f_match_level == 3 {
                env_key.push(c);
              } else if f_match_level == 2 {
                env_key = "".to_owned();
              }
              if f_match_level > 0 && f_match_level < 4 {
                partial_match_idx = Some(idx);
              }
          }
          // tmp.push_str(unsafe { target.get_unchecked(last_end..end_i) });

          tx.send(ChannelValue {
              a: i,
              partial_match_idx,
              text: "".to_owned(),
              replacement: tmp,
          })
          .unwrap(); // (非同期)送信
      });
  }
  // end
  let mut hmap: HashMap<usize, ChannelValue, BuildHasherDefault<FxHasher>> = HashMap::default();

  // receive channel
  println!("receive channel");
  for i in rx.iter().take(thread_n) {
      hmap.insert(i.a, i);
  }
  // partial match resolve
  println!("partial match resolve");
  let target_str = target.as_str();
  for i in 0..(thread_n) {
    if let Some(a) = hmap.get_mut(&i) {
      if let Some(partial_match_idx) = a.partial_match_idx {
        let mut f_match_level = 0;
        let mut env_key_start = 0;

        // until matched index
        let mut j = partial_match_idx;
        loop {
          let b = unsafe { target_str.get_unchecked(j..j+1) };
          // FIXME:
          (f_match_level, env_key_start) = replace(j, b.chars().next().unwrap(), f_match_level, env_key_start);
          if f_match_level == 4 {
            if let Some(val) = envvars.get(unsafe { target_str.get_unchecked(env_key_start..j) }) {
              a.replacement.push((env_key_start-2..j+1, val.to_owned()));
            }
            break;
          } else if f_match_level == 0 {
            break;
          }
          j += 1
        }
      }
    }
  }
  // concat
  println!("concat");
  let mut start_i = 0;
  for i in 0..(thread_n) {
      if let Some(a) = hmap.get(&i) {
        for (rng, val) in &a.replacement {
          result.push_str(unsafe { target_str.get_unchecked(start_i..rng.start) });
          result.push_str(val);
          start_i = rng.end;
        }
      }
  }
  result.push_str(unsafe { target_str.get_unchecked(start_i..char_count) });
  println!("done!");
  result
}

fn replace(j: usize, c: char, f_match_level: usize, env_key_start: usize) -> (usize, usize) {
  // search
  if c == '$' {
      return (1, 0)
  } else if c == '{' {
      if f_match_level == 1 {
          return (2, 0)
      } else {
          return (0, 0)
      }
  } else if c == '_' || c.is_alphanumeric() {
      if f_match_level == 2 {
        return (3, j)
      } else if f_match_level == 3 {
        return (3, env_key_start)
      } else {
        return (0, 0)
      }
  } else if c == '}' && f_match_level == 3 {
      return (4, env_key_start)
  } else {
      return (0, 0)
  }
}

#[cfg(test)]
mod tests {
    use crate::{
        model::EnvVar,
        replace_dollar_braces::{replace_dollar_braces, replace_dollar_braces_with_hashmap, replace_dollar_braces_with_hashmap_parallel},
    };
    use rustc_hash::FxHasher;
    use std::{collections::HashMap, hash::BuildHasherDefault, sync::{Mutex, Arc}};

    #[test]
    fn vec() {
        let envvars: Vec<EnvVar> = vec![
            EnvVar {
                key: "KEY1".to_owned(),
                val: "value1".to_owned(),
            },
            EnvVar {
                key: "KEY2".to_owned(),
                val: "value2".to_owned(),
            },
            EnvVar {
                key: "KEY3".to_owned(),
                val: "value3".to_owned(),
            },
            EnvVar {
                key: "KEY4".to_owned(),
                val: "value4".to_owned(),
            },
            EnvVar {
                key: "KEY5".to_owned(),
                val: "value5".to_owned(),
            },
            EnvVar {
                key: "KEY6".to_owned(),
                val: "value6".to_owned(),
            },
            EnvVar {
                key: "KEY7".to_owned(),
                val: "value7".to_owned(),
            },
            EnvVar {
                key: "KEY8".to_owned(),
                val: "value8".to_owned(),
            },
            EnvVar {
                key: "KEY9".to_owned(),
                val: "value9".to_owned(),
            },
            EnvVar {
                key: "KEY10".to_owned(),
                val: "value10".to_owned(),
            },
        ];
        let target =
            "2fwa${KEY1}hfasd${KEY1}fnadnfa0${KEY1}2fwah${KEY1}fasdf${KEY1}na${KEY1}${KEY1}dnfa0
      2fwa${KEY2}hfasd${KEY2}fnadnfa0${KEY2}2fwah${KEY2}fasdf${KEY2}na${KEY2}${KEY2}dnfa0
      2fwa${KEY3}hfasd${KEY3}fnadnfa0${KEY3}2fwah${KEY3}fasdf${KEY3}na${KEY3}${KEY3}dnfa0
      2fwa${KEY4}hfasd${KEY4}fnadnfa0${KEY4}2fwah${KEY4}fasdf${KEY4}na${KEY4}${KEY4}dnfa0
      2fwa${KEY5}hfasd${KEY5}fnadnfa0${KEY5}2fwah${KEY5}fasdf${KEY5}na${KEY5}${KEY5}dnfa0
      2fwa${KEY6}hfasd${KEY6}fnadnfa0${KEY6}2fwah${KEY6}fa😹sdf${KEY6}na${KEY6}${KEY6}dnfa0
      2fwa${KEY7}hfasd${KEY7}fnadnfa0${KEY7}2fwah${KEY7}fasdf${KEY7}na${KEY7}${KEY7}dnfa0
      2fwa${KEY8}hfasd${KEY8}fnadnfa0${KEY8}2fwah${KEY8}fasdf${KEY8}na${KEY8}${KEY8}dnfa0
      2fwa${KEY9}hfasd${KEY9}fnadnfa0${KEY9}2fwah${KEY9}fasdf${KEY9}na${KEY9}${KEY9}dnfa0
      2fwah${KEY10}fasd${KEY10}fnadnfa0$${KEY10}2fwa${KEY10}hfasd${KEY10}fn${KEY10}${KEY10}adnfa";
        let expected =
            "2fwavalue1hfasdvalue1fnadnfa0value12fwahvalue1fasdfvalue1navalue1value1dnfa0
      2fwavalue2hfasdvalue2fnadnfa0value22fwahvalue2fasdfvalue2navalue2value2dnfa0
      2fwavalue3hfasdvalue3fnadnfa0value32fwahvalue3fasdfvalue3navalue3value3dnfa0
      2fwavalue4hfasdvalue4fnadnfa0value42fwahvalue4fasdfvalue4navalue4value4dnfa0
      2fwavalue5hfasdvalue5fnadnfa0value52fwahvalue5fasdfvalue5navalue5value5dnfa0
      2fwavalue6hfasdvalue6fnadnfa0value62fwahvalue6fa😹sdfvalue6navalue6value6dnfa0
      2fwavalue7hfasdvalue7fnadnfa0value72fwahvalue7fasdfvalue7navalue7value7dnfa0
      2fwavalue8hfasdvalue8fnadnfa0value82fwahvalue8fasdfvalue8navalue8value8dnfa0
      2fwavalue9hfasdvalue9fnadnfa0value92fwahvalue9fasdfvalue9navalue9value9dnfa0
      2fwahvalue10fasdvalue10fnadnfa0$value102fwavalue10hfasdvalue10fnvalue10value10adnfa";

        let ret = replace_dollar_braces(&envvars, target.to_owned());
        assert_eq!(ret, expected);
    }

    #[test]
    fn hashmap() {
        let mut envvars: HashMap<String, String, BuildHasherDefault<FxHasher>> = HashMap::default();
        envvars.insert("KEY1".to_owned(), "value1".to_owned());
        envvars.insert("KEY2".to_owned(), "value2".to_owned());
        envvars.insert("KEY3".to_owned(), "value3".to_owned());
        envvars.insert("KEY4".to_owned(), "value4".to_owned());
        envvars.insert("KEY5".to_owned(), "value5".to_owned());
        envvars.insert("KEY6".to_owned(), "value6".to_owned());
        envvars.insert("KEY7".to_owned(), "value7".to_owned());
        envvars.insert("KEY8".to_owned(), "value8".to_owned());
        envvars.insert("KEY9".to_owned(), "value9".to_owned());
        envvars.insert("KEY10".to_owned(), "value10".to_owned());
        let target =
          "2fwa${KEY1}hfasd${KEY1}fnadnfa0${KEY1}2fwah${KEY1}fasdf${KEY1}na${KEY1}${KEY1}dnfa0
      2fwa${KEY2}hfasd${KEY2}fnadnfa0${KEY2}2fwah${KEY2}fasdf${KEY2}na${KEY2}${KEY2}dnfa0
      2fwa${KEY3}hfasd${KEY3}fnadnfa0${KEY3}2fwah${KEY3}fasdf${KEY3}na${KEY3}${KEY3}dnfa0
      2fwa${KEY4}hfasd${KEY4}fnadnfa0${KEY4}2fwah${KEY4}fasdf${KEY4}na${KEY4}${KEY4}dnfa0
      2fwa${KEY5}hfasd${KEY5}fnadnfa0${KEY5}2fwah${KEY5}fasdf${KEY5}na${KEY5}${KEY5}dnfa0
      2fwa${KEY6}hfasd${KEY6}fnadnfa0${KEY6}2fwah${KEY6}fa😹sdf${KEY6}na${KEY6}${KEY6}dnfa0
      2fwa${KEY7}hfasd${KEY7}fnadnfa0${KEY7}2fwah${KEY7}fasdf${KEY7}na${KEY7}${KEY7}dnfa0
      2fwa${KEY8}hfasd${KEY8}fnadnfa0${KEY8}2fwah${KEY8}fasdf${KEY8}na${KEY8}${KEY8}dnfa0
      2fwa${KEY9}hfasd${KEY9}fnadnfa0${KEY9}2fwah${KEY9}fasdf${KEY9}na${KEY9}${KEY9}dnfa0
      2fwah${KEY10}fasd${KEY10}fnadnfa0${{$${KEY10}2fwa${KEY10}hfasd${KEY10}fn${KEY10}${KEY10}adnfa";
        let expected =
            "2fwavalue1hfasdvalue1fnadnfa0value12fwahvalue1fasdfvalue1navalue1value1dnfa0
      2fwavalue2hfasdvalue2fnadnfa0value22fwahvalue2fasdfvalue2navalue2value2dnfa0
      2fwavalue3hfasdvalue3fnadnfa0value32fwahvalue3fasdfvalue3navalue3value3dnfa0
      2fwavalue4hfasdvalue4fnadnfa0value42fwahvalue4fasdfvalue4navalue4value4dnfa0
      2fwavalue5hfasdvalue5fnadnfa0value52fwahvalue5fasdfvalue5navalue5value5dnfa0
      2fwavalue6hfasdvalue6fnadnfa0value62fwahvalue6fa😹sdfvalue6navalue6value6dnfa0
      2fwavalue7hfasdvalue7fnadnfa0value72fwahvalue7fasdfvalue7navalue7value7dnfa0
      2fwavalue8hfasdvalue8fnadnfa0value82fwahvalue8fasdfvalue8navalue8value8dnfa0
      2fwavalue9hfasdvalue9fnadnfa0value92fwahvalue9fasdfvalue9navalue9value9dnfa0
      2fwahvalue10fasdvalue10fnadnfa0${{$value102fwavalue10hfasdvalue10fnvalue10value10adnfa";

        let ret = replace_dollar_braces_with_hashmap(&envvars, target);
        assert_eq!(ret.to_owned(), expected.to_owned());
    }

    #[test]
    fn hashmap_parallel() {
        let mut envvars: HashMap<String, String, BuildHasherDefault<FxHasher>> = HashMap::default();
        envvars.insert("KEY1".to_owned(), "value1".to_owned());
        envvars.insert("KEY2".to_owned(), "value2".to_owned());
        envvars.insert("KEY3".to_owned(), "value3".to_owned());
        envvars.insert("KEY4".to_owned(), "value4".to_owned());
        envvars.insert("KEY5".to_owned(), "value5".to_owned());
        envvars.insert("KEY6".to_owned(), "value6".to_owned());
        envvars.insert("KEY7".to_owned(), "value7".to_owned());
        envvars.insert("KEY8".to_owned(), "value8".to_owned());
        envvars.insert("KEY9".to_owned(), "value9".to_owned());
        envvars.insert("KEY10".to_owned(), "value10".to_owned());
        let target =
          "2fwa${KEY1}hfasd${KEY1}fnadnfa0${KEY1}2fwah${KEY1}fasdf${KEY1}na${KEY1}${KEY1}dnfa0
      2fwa${KEY2}hfasd${KEY2}fnadnfa0${KEY2}2fwah${KEY2}fasdf${KEY2}na${KEY2}${KEY2}dnfa0
      2fwa${KEY3}hfasd${KEY3}fnadnfa0${KEY3}2fwah${KEY3}fasdf${KEY3}na${KEY3}${KEY3}dnfa0
      2fwa${KEY4}hfasd${KEY4}fnadnfa0${KEY4}2fwah${KEY4}fasdf${KEY4}na${KEY4}${KEY4}dnfa0
      2fwa${KEY5}hfasd${KEY5}fnadnfa0${KEY5}2fwah${KEY5}fasdf${KEY5}na${KEY5}${KEY5}dnfa0
      2fwa${KEY6}hfasd${KEY6}fnadnfa0${KEY6}2fwah${KEY6}fa😹sdf${KEY6}na${KEY6}${KEY6}dnfa0
      2fwa${KEY7}hfasd${KEY7}fnadnfa0${KEY7}2fwah${KEY7}fasdf${KEY7}na${KEY7}${KEY7}dnfa0
      2fwa${KEY8}hfasd${KEY8}fnadnfa0${KEY8}2fwah${KEY8}fasdf${KEY8}na${KEY8}${KEY8}dnfa0
      2fwa${KEY9}hfasd${KEY9}fnadnfa0${KEY9}2fwah${KEY9}fasdf${KEY9}na${KEY9}${KEY9}dnfa0
      2fwah${KEY10}fasd${KEY10}fnadnfa0${{$${KEY10}2fwa${KEY10}hfasd${KEY10}fn${KEY10}${KEY10}adnfa";
        let expected =
            "2fwavalue1hfasdvalue1fnadnfa0value12fwahvalue1fasdfvalue1navalue1value1dnfa0
      2fwavalue2hfasdvalue2fnadnfa0value22fwahvalue2fasdfvalue2navalue2value2dnfa0
      2fwavalue3hfasdvalue3fnadnfa0value32fwahvalue3fasdfvalue3navalue3value3dnfa0
      2fwavalue4hfasdvalue4fnadnfa0value42fwahvalue4fasdfvalue4navalue4value4dnfa0
      2fwavalue5hfasdvalue5fnadnfa0value52fwahvalue5fasdfvalue5navalue5value5dnfa0
      2fwavalue6hfasdvalue6fnadnfa0value62fwahvalue6fa😹sdfvalue6navalue6value6dnfa0
      2fwavalue7hfasdvalue7fnadnfa0value72fwahvalue7fasdfvalue7navalue7value7dnfa0
      2fwavalue8hfasdvalue8fnadnfa0value82fwahvalue8fasdfvalue8navalue8value8dnfa0
      2fwavalue9hfasdvalue9fnadnfa0value92fwahvalue9fasdfvalue9navalue9value9dnfa0
      2fwahvalue10fasdvalue10fnadnfa0${{$value102fwavalue10hfasdvalue10fnvalue10value10adnfa";

        let ret = replace_dollar_braces_with_hashmap_parallel(&envvars, target.to_owned(), 24);
        assert_eq!(ret.to_owned(), expected.to_owned());
    }
}
