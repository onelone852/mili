use serde::ser::SerializeMap;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Localization {
    original: Box<str>,
    #[cfg(feature = "localization")]
    localizations: LocalizationMap,
}

impl Localization {
    #[inline]
    pub fn plain(text: impl Into<Box<str>>) -> Self {
        Self {
            original: text.into(),
            localizations: LocalizationMap::default(),
        }
    }

    #[inline]
    pub fn localization_with<'a>(&'a self, name: &'a str) -> impl Serialize + 'a {
        struct Serializer<'a>(&'a Localization, &'a str);

        #[cfg(feature = "localization")]
        impl<'a> Serialize for Serializer<'a> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry(self.1, &self.0.original)?;

                let key = {
                    let mut key = self.1.to_string();
                    key.push_str("_localizations");
                    key
                };
                map.serialize_entry(&key, &self.0.localizations)?;

                map.end()
            }
        }

        #[cfg(not(feature = "localization"))]
        impl<'a> Serialize for Serializer<'a> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry(self.1, &self.0.original)?;
                map.end()
            }
        }

        Serializer(self, name)
    }
}

impl<T> From<T> for Localization
where
    T: Into<Box<str>>,
{
    #[inline]
    fn from(value: T) -> Self {
        Self::plain(value)
    }
}

macro_rules! local_methods_init {
    ($lang:ident) => {
        #[inline]
        pub fn $lang(mut self, text: impl Into<Box<str>>) -> Self {
            self.localizations = self.localizations.$lang(text);
            self
        }
    };
    ($lang:ident, $($langs:ident), +) => {
       local_methods_init!($lang);
       local_methods_init!($($langs), +);
    };
}

macro_rules! local_map_methods_init {
    ($lang:ident) => {
        #[inline]
        pub fn $lang(mut self, text: impl Into<Box<str>>) -> Self {
            self.$lang = Some(text.into());
            self
        }
    };
    ($lang:ident, $($langs:ident), +) => {
       local_map_methods_init!($lang);
       local_map_methods_init!($($langs), +);
    };
}

#[cfg(feature = "localization")]
impl Localization {
    pub fn map(mut self, map: LocalizationMap) -> Self {
        self.localizations = map;
        self
    }

    local_methods_init!(
        id, da, de, en_gb, en_us, es_es, fr, hr, it, lt, hu, nl, no, pl, pt_br, ro, fi, sv_se, vi,
        tr, cs, el, bg, ru, uk, hi, th, zh_cn, ja, zh_tw, ko
    );
}

#[cfg(feature = "localization")]
#[derive(Debug, Clone, Serialize, Default)]
pub struct LocalizationMap {
    pub id: Option<Box<str>>,
    pub da: Option<Box<str>>,
    pub de: Option<Box<str>>,
    #[serde(rename = "en-GB")]
    pub en_gb: Option<Box<str>>,
    #[serde(rename = "en-US")]
    pub en_us: Option<Box<str>>,
    #[serde(rename = "es-ES")]
    pub es_es: Option<Box<str>>,
    pub fr: Option<Box<str>>,
    pub hr: Option<Box<str>>,
    pub it: Option<Box<str>>,
    pub lt: Option<Box<str>>,
    pub hu: Option<Box<str>>,
    pub nl: Option<Box<str>>,
    pub no: Option<Box<str>>,
    pub pl: Option<Box<str>>,
    #[serde(rename = "pt-BR")]
    pub pt_br: Option<Box<str>>,
    pub ro: Option<Box<str>>,
    pub fi: Option<Box<str>>,
    #[serde(rename = "sv-SE")]
    pub sv_se: Option<Box<str>>,
    pub vi: Option<Box<str>>,
    pub tr: Option<Box<str>>,
    pub cs: Option<Box<str>>,
    pub el: Option<Box<str>>,
    pub bg: Option<Box<str>>,
    pub ru: Option<Box<str>>,
    pub uk: Option<Box<str>>,
    pub hi: Option<Box<str>>,
    pub th: Option<Box<str>>,
    #[serde(rename = "zh-CN")]
    pub zh_cn: Option<Box<str>>,
    pub ja: Option<Box<str>>,
    #[serde(rename = "zh-TW")]
    pub zh_tw: Option<Box<str>>,
    pub ko: Option<Box<str>>,
}

#[cfg(feature = "localization")]
impl LocalizationMap {
    local_map_methods_init!(
        id, da, de, en_gb, en_us, es_es, fr, hr, it, lt, hu, nl, no, pl, pt_br, ro, fi, sv_se, vi,
        tr, cs, el, bg, ru, uk, hi, th, zh_cn, ja, zh_tw, ko
    );
}
