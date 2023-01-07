use criterion::{criterion_group, criterion_main, Criterion};

use envbed::model::EnvVar;
use envbed::replace_dollar_braces::{replace_dollar_braces, replace_dollar_braces_with_hashmap};
use envbed::replace_double_braces::{replace_double_braces, replace_double_braces_with_hashmap};
use rustc_hash::FxHasher;
use std::{collections::HashMap, hash::BuildHasherDefault};

fn bench_dollar_vec(c: &mut Criterion) {
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
  2fwa${KEY6}hfasd${KEY6}fnadnfa0${KEY6}2fwah${KEY6}fasdf${KEY6}na${KEY6}${KEY6}dnfa0
  2fwa${KEY7}hfasd${KEY7}fnadnfa0${KEY7}2fwah${KEY7}fasdf${KEY7}na${KEY7}${KEY7}dnfa0
  2fwa${KEY8}hfasd${KEY8}fnadnfa0${KEY8}2fwah${KEY8}fasdf${KEY8}na${KEY8}${KEY8}dnfa0
  2fwa${KEY9}hfasd${KEY9}fnadnfa0${KEY9}2fwah${KEY9}fasdf${KEY9}na${KEY9}${KEY9}dnfa0
  2fwah${KEY10}fasd${KEY10}fnadnfa0$${KEY10}2fwa${KEY10}hfasd${KEY10}fn${KEY10}${KEY10}adnfa";

    c.bench_function("replace_dollar_braces_process", |b| {
        b.iter(|| replace_dollar_braces(&envvars, target.to_owned()))
    });
}

fn bench_dollar_hash(c: &mut Criterion) {
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
  2fwa${KEY6}hfasd${KEY6}fnadnfa0${KEY6}2fwah${KEY6}fasdf${KEY6}na${KEY6}${KEY6}dnfa0
  2fwa${KEY7}hfasd${KEY7}fnadnfa0${KEY7}2fwah${KEY7}fasdf${KEY7}na${KEY7}${KEY7}dnfa0
  2fwa${KEY8}hfasd${KEY8}fnadnfa0${KEY8}2fwah${KEY8}fasdf${KEY8}na${KEY8}${KEY8}dnfa0
  2fwa${KEY9}hfasd${KEY9}fnadnfa0${KEY9}2fwah${KEY9}fasdf${KEY9}na${KEY9}${KEY9}dnfa0
  2fwah${KEY10}fasd${KEY10}fnadnfa0$${KEY10}2fwa${KEY10}hfasd${KEY10}fn${KEY10}${KEY10}adnfa";

    c.bench_function("replace_dollar_braces_with_hashmap_process", |b| {
        b.iter(|| replace_dollar_braces_with_hashmap(&envvars, target))
    });
}

fn bench_double_vec(c: &mut Criterion) {
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
  2fwa${KEY6}hfasd${KEY6}fnadnfa0${KEY6}2fwah${KEY6}fasdf${KEY6}na${KEY6}${KEY6}dnfa0
  2fwa${KEY7}hfasd${KEY7}fnadnfa0${KEY7}2fwah${KEY7}fasdf${KEY7}na${KEY7}${KEY7}dnfa0
  2fwa${KEY8}hfasd${KEY8}fnadnfa0${KEY8}2fwah${KEY8}fasdf${KEY8}na${KEY8}${KEY8}dnfa0
  2fwa${KEY9}hfasd${KEY9}fnadnfa0${KEY9}2fwah${KEY9}fasdf${KEY9}na${KEY9}${KEY9}dnfa0
  2fwah${KEY10}fasd${KEY10}fnadnfa0$${KEY10}2fwa${KEY10}hfasd${KEY10}fn${KEY10}${KEY10}adnfa";

    c.bench_function("replace_double_braces_process", |b| {
        b.iter(|| replace_double_braces(&envvars, target.to_string()))
    });
}

fn bench_double_hash(c: &mut Criterion) {
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
  2fwa${KEY6}hfasd${KEY6}fnadnfa0${KEY6}2fwah${KEY6}fasdf${KEY6}na${KEY6}${KEY6}dnfa0
  2fwa${KEY7}hfasd${KEY7}fnadnfa0${KEY7}2fwah${KEY7}fasdf${KEY7}na${KEY7}${KEY7}dnfa0
  2fwa${KEY8}hfasd${KEY8}fnadnfa0${KEY8}2fwah${KEY8}fasdf${KEY8}na${KEY8}${KEY8}dnfa0
  2fwa${KEY9}hfasd${KEY9}fnadnfa0${KEY9}2fwah${KEY9}fasdf${KEY9}na${KEY9}${KEY9}dnfa0
  2fwah${KEY10}fasd${KEY10}fnadnfa0$${KEY10}2fwa${KEY10}hfasd${KEY10}fn${KEY10}${KEY10}adnfa";

    c.bench_function("replace_double_braces_with_hashmap", |b| {
        b.iter(|| replace_double_braces_with_hashmap(&envvars, target))
    });
}

criterion_group!(
    benches,
    bench_dollar_vec,
    bench_dollar_hash,
    bench_double_vec,
    bench_double_hash
);
criterion_main!(benches);
