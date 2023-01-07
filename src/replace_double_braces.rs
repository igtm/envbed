use crate::model::EnvVar;
use rustc_hash::FxHasher;
use std::{collections::HashMap, hash::BuildHasherDefault};

#[allow(dead_code)]
pub fn replace_double_braces(envvars: &Vec<EnvVar>, mut target: String) -> String {
    for envvar in envvars {
        target = target.replace(&format!("{{{{{}}}}}", &envvar.key), &envvar.val)
    }

    target
}

#[allow(dead_code)]
pub fn replace_double_braces_with_hashmap(
    envvars: &HashMap<String, String, BuildHasherDefault<FxHasher>>,
    target: &str,
) -> String {
    let mut result = String::new();
    let mut last_end = 0;
    let mut env_key_start = 0;

    let mut f_match_level: usize = 0; // 0,1={,2={{,3={{},4={{}}
    for (i, c) in target.char_indices() {
        if c == '{' {
            if f_match_level == 0 {
                f_match_level = 1;
            } else if f_match_level == 1 || f_match_level == 2 {
                f_match_level = 2;
            } else {
                f_match_level = 1;
            }
        } else if c == '_' || c.is_alphanumeric() {
            if f_match_level == 2 {
                result.push_str(unsafe { target.get_unchecked(last_end..(i - 2)) });
                last_end = i - 2;
                f_match_level = 3;
                env_key_start = i;
            }
        } else if c == '}' {
            if f_match_level == 3 {
                f_match_level = 4
            } else if f_match_level == 4 {
                // get value from hashmap
                if let Some(val) =
                    envvars.get(unsafe { target.get_unchecked(env_key_start..(i - 1)) })
                {
                    result.push_str(val);
                    last_end = i + 1;
                }
                env_key_start = 0;
                f_match_level = 0;
            } else {
                env_key_start = 0;
                f_match_level = 0;
            }
        } else {
            env_key_start = 0;
            f_match_level = 0;
        }
    }
    result.push_str(unsafe { target.get_unchecked(last_end..target.len()) });
    result
}

#[cfg(test)]
mod tests {
    use crate::{
        model::EnvVar,
        replace_double_braces::{replace_double_braces, replace_double_braces_with_hashmap},
    };
    use rustc_hash::FxHasher;
    use std::{collections::HashMap, hash::BuildHasherDefault};

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
            "2fwa{{KEY1}}hfasd{{KEY1}}fnadnfa0{{KEY1}}2fwah{{KEY1}}fasdf{{KEY1}}na{{KEY1}}{{KEY1}}dnfa0
      2fwa{{KEY2}}hfasd{{KEY2}}fnadnfa0{{KEY2}}2fwah{{KEY2}}fasdf{{KEY2}}na{{KEY2}}{{KEY2}}dnfa0
      2fwa{{KEY3}}hfasd{{KEY3}}fnadnfa0{{KEY3}}2fwah{{KEY3}}fasdf{{KEY3}}na{{KEY3}}{{KEY3}}dnfa0
      2fwa{{KEY4}}hfasd{{KEY4}}fnadnfa0{{KEY4}}2fwah{{KEY4}}fasdf{{KEY4}}na{{KEY4}}{{KEY4}}dnfa0
      2fwa{{KEY5}}hfasd{{KEY5}}fnadnfa0{{KEY5}}2fwah{{KEY5}}fasdf{{KEY5}}na{{KEY5}}{{KEY5}}dnfa0
      2fwa{{KEY6}}hfasd{{KEY6}}fnadnfa0{{KEY6}}2fwah{{KEY6}}fasdf{{KEY6}}na{{KEY6}}{{KEY6}}dnfa0
      2fwa{{KEY7}}hfasd{{KEY7}}fnadnfa0{{KEY7}}2fwah{{KEY7}}fasdf{{KEY7}}na{{KEY7}}{{KEY7}}dnfa0
      2fwa{{KEY8}}hfasd{{KEY8}}fnadnfa0{{KEY8}}2fwah{{KEY8}}fasdf{{KEY8}}na{{KEY8}}{{KEY8}}dnfa0
      2fwa{{KEY9}}hfasd{{KEY9}}fnadnfa0{{KEY9}}2fwah{{KEY9}}fasdf{{KEY9}}na{{KEY9}}{{KEY9}}dnfa0
      2fwah{{KEY10}}fasd{{KEY10}}fnadnfa0${{KEY10}}2fwa{{KEY10}}hfasd{{KEY10}}fn{{KEY10}}{{KEY10}}adnfa";
        let expected =
            "2fwavalue1hfasdvalue1fnadnfa0value12fwahvalue1fasdfvalue1navalue1value1dnfa0
      2fwavalue2hfasdvalue2fnadnfa0value22fwahvalue2fasdfvalue2navalue2value2dnfa0
      2fwavalue3hfasdvalue3fnadnfa0value32fwahvalue3fasdfvalue3navalue3value3dnfa0
      2fwavalue4hfasdvalue4fnadnfa0value42fwahvalue4fasdfvalue4navalue4value4dnfa0
      2fwavalue5hfasdvalue5fnadnfa0value52fwahvalue5fasdfvalue5navalue5value5dnfa0
      2fwavalue6hfasdvalue6fnadnfa0value62fwahvalue6fasdfvalue6navalue6value6dnfa0
      2fwavalue7hfasdvalue7fnadnfa0value72fwahvalue7fasdfvalue7navalue7value7dnfa0
      2fwavalue8hfasdvalue8fnadnfa0value82fwahvalue8fasdfvalue8navalue8value8dnfa0
      2fwavalue9hfasdvalue9fnadnfa0value92fwahvalue9fasdfvalue9navalue9value9dnfa0
      2fwahvalue10fasdvalue10fnadnfa0$value102fwavalue10hfasdvalue10fnvalue10value10adnfa";

        let ret = replace_double_braces(&envvars, target.to_owned());
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
            "2fwa{{KEY1}}hfasd{{KEY1}}fnadnfa0{{KEY1}}2fwah{{KEY1}}fasdf{{KEY1}}na{{KEY1}}{{KEY1}}dnfa0
      2fwa{{KEY2}}hfasd{{KEY2}}fnadnfa0{{KEY2}}2fwah{{KEY2}}fasdf{{KEY2}}na{{KEY2}}{{KEY2}}dnfa0
      2fwa{{KEY3}}hfasd{{KEY3}}fnadnfa0{{KEY3}}2fwah{{KEY3}}fasdf{{KEY3}}na{{KEY3}}{{KEY3}}dnfa0
      2fwa{{KEY4}}hfasd{{KEY4}}fnadnfa0{{KEY4}}2fwah{{KEY4}}fasdf{{KEY4}}na{{KEY4}}{{KEY4}}dnfa0
      2fwa{{KEY5}}hfasd{{KEY5}}fnadnfa0{{KEY5}}2fwah{{KEY5}}fasdf{{KEY5}}na{{KEY5}}{{KEY5}}dnfa0
      2fwa{{KEY6}}hfasd{{KEY6}}fnadnfa0{{KEY6}}2fwah{{KEY6}}fasdf{{KEY6}}na{{KEY6}}{{KEY6}}dnfa0
      2fwa{{KEY7}}hfasd{{KEY7}}fnadnfa0{{KEY7}}2fwah{{KEY7}}fasdf{{KEY7}}na{{KEY7}}{{KEY7}}dnfa0
      2fwa{{KEY8}}hfasd{{KEY8}}fnadnfa0{{KEY8}}2fwah{{KEY8}}fasdf{{KEY8}}na{{KEY8}}{{KEY8}}dnfa0
      2fwa{{KEY9}}hfasd{{KEY9}}fnadnfa0{{KEY9}}2fwah{{KEY9}}fasdf{{KEY9}}na{{KEY9}}{{KEY9}}dnfa0
      2fwah{{KEY10}}fasd{{KEY10}}fnad{{}}}}{{{{nfa0${{${{KEY10}}2fwa{{KEY10}}hfasd{{KEY10}}fn{{KEY10}}{{KEY10}}adnfa";
        let expected =
            "2fwavalue1hfasdvalue1fnadnfa0value12fwahvalue1fasdfvalue1navalue1value1dnfa0
      2fwavalue2hfasdvalue2fnadnfa0value22fwahvalue2fasdfvalue2navalue2value2dnfa0
      2fwavalue3hfasdvalue3fnadnfa0value32fwahvalue3fasdfvalue3navalue3value3dnfa0
      2fwavalue4hfasdvalue4fnadnfa0value42fwahvalue4fasdfvalue4navalue4value4dnfa0
      2fwavalue5hfasdvalue5fnadnfa0value52fwahvalue5fasdfvalue5navalue5value5dnfa0
      2fwavalue6hfasdvalue6fnadnfa0value62fwahvalue6fasdfvalue6navalue6value6dnfa0
      2fwavalue7hfasdvalue7fnadnfa0value72fwahvalue7fasdfvalue7navalue7value7dnfa0
      2fwavalue8hfasdvalue8fnadnfa0value82fwahvalue8fasdfvalue8navalue8value8dnfa0
      2fwavalue9hfasdvalue9fnadnfa0value92fwahvalue9fasdfvalue9navalue9value9dnfa0
      2fwahvalue10fasdvalue10fnad{{}}}}{{{{nfa0${{$value102fwavalue10hfasdvalue10fnvalue10value10adnfa";

        let ret = replace_double_braces_with_hashmap(&envvars, target);
        assert_eq!(ret.to_owned(), expected.to_owned());
    }
}
