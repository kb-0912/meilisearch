//! All the structures used to make the analytics on the settings works.
//! The signatures of the `new` functions are not very rust idiomatic because they must match the types received
//! through the sub-settings route directly without any manipulation.
//! This is why we often use a `Option<&Vec<_>>` instead of a `Option<&[_]>`.

use meilisearch_types::locales::{Locale, LocalizedAttributesRuleView};
use meilisearch_types::milli::update::Setting;
use meilisearch_types::milli::vector::settings::EmbeddingSettings;
use meilisearch_types::settings::{
    FacetingSettings, PaginationSettings, ProximityPrecisionView, TypoSettings,
};
use meilisearch_types::{facet_values_sort::FacetValuesSort, settings::RankingRuleView};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::analytics::Aggregate;

#[derive(Serialize, Default)]
pub struct SettingsAnalytics {
    pub ranking_rules: RankingRulesAnalytics,
    pub searchable_attributes: SearchableAttributesAnalytics,
    pub displayed_attributes: DisplayedAttributesAnalytics,
    pub sortable_attributes: SortableAttributesAnalytics,
    pub filterable_attributes: FilterableAttributesAnalytics,
    pub distinct_attribute: DistinctAttributeAnalytics,
    pub proximity_precision: ProximityPrecisionAnalytics,
    pub typo_tolerance: TypoToleranceAnalytics,
    pub faceting: FacetingAnalytics,
    pub pagination: PaginationAnalytics,
    pub stop_words: StopWordsAnalytics,
    pub synonyms: SynonymsAnalytics,
    pub embedders: EmbeddersAnalytics,
    pub search_cutoff_ms: SearchCutoffMsAnalytics,
    pub locales: LocalesAnalytics,
    pub dictionary: DictionaryAnalytics,
    pub separator_tokens: SeparatorTokensAnalytics,
    pub non_separator_tokens: NonSeparatorTokensAnalytics,
}

impl Aggregate for SettingsAnalytics {
    fn event_name(&self) -> &'static str {
        "Settings Updated"
    }

    fn aggregate(self: Box<Self>, other: Box<Self>) -> Box<Self> {
        Box::new(Self {
            ranking_rules: RankingRulesAnalytics {
                words_position: self
                    .ranking_rules
                    .words_position
                    .or(other.ranking_rules.words_position),
                typo_position: self
                    .ranking_rules
                    .typo_position
                    .or(other.ranking_rules.typo_position),
                proximity_position: self
                    .ranking_rules
                    .proximity_position
                    .or(other.ranking_rules.proximity_position),
                attribute_position: self
                    .ranking_rules
                    .attribute_position
                    .or(other.ranking_rules.attribute_position),
                sort_position: self
                    .ranking_rules
                    .sort_position
                    .or(other.ranking_rules.sort_position),
                exactness_position: self
                    .ranking_rules
                    .exactness_position
                    .or(other.ranking_rules.exactness_position),
                values: self.ranking_rules.values.or(other.ranking_rules.values),
            },
            searchable_attributes: SearchableAttributesAnalytics {
                total: self.searchable_attributes.total.or(other.searchable_attributes.total),
                with_wildcard: self
                    .searchable_attributes
                    .with_wildcard
                    .or(other.searchable_attributes.with_wildcard),
            },
            displayed_attributes: DisplayedAttributesAnalytics {
                total: self.displayed_attributes.total.or(other.displayed_attributes.total),
                with_wildcard: self
                    .displayed_attributes
                    .with_wildcard
                    .or(other.displayed_attributes.with_wildcard),
            },
            sortable_attributes: SortableAttributesAnalytics {
                total: self.sortable_attributes.total.or(other.sortable_attributes.total),
                has_geo: self.sortable_attributes.has_geo.or(other.sortable_attributes.has_geo),
            },
            filterable_attributes: FilterableAttributesAnalytics {
                total: self.filterable_attributes.total.or(other.filterable_attributes.total),
                has_geo: self.filterable_attributes.has_geo.or(other.filterable_attributes.has_geo),
            },
            distinct_attribute: DistinctAttributeAnalytics {
                set: self.distinct_attribute.set | other.distinct_attribute.set,
            },
            proximity_precision: ProximityPrecisionAnalytics {
                set: self.proximity_precision.set | other.proximity_precision.set,
                value: self.proximity_precision.value.or(other.proximity_precision.value),
            },
            typo_tolerance: TypoToleranceAnalytics {
                enabled: self.typo_tolerance.enabled.or(other.typo_tolerance.enabled),
                disable_on_attributes: self
                    .typo_tolerance
                    .disable_on_attributes
                    .or(other.typo_tolerance.disable_on_attributes),
                disable_on_words: self
                    .typo_tolerance
                    .disable_on_words
                    .or(other.typo_tolerance.disable_on_words),
                min_word_size_for_one_typo: self
                    .typo_tolerance
                    .min_word_size_for_one_typo
                    .or(other.typo_tolerance.min_word_size_for_one_typo),
                min_word_size_for_two_typos: self
                    .typo_tolerance
                    .min_word_size_for_two_typos
                    .or(other.typo_tolerance.min_word_size_for_two_typos),
            },
            faceting: FacetingAnalytics {
                max_values_per_facet: self
                    .faceting
                    .max_values_per_facet
                    .or(other.faceting.max_values_per_facet),
                sort_facet_values_by_star_count: self
                    .faceting
                    .sort_facet_values_by_star_count
                    .or(other.faceting.sort_facet_values_by_star_count),
                sort_facet_values_by_total: self
                    .faceting
                    .sort_facet_values_by_total
                    .or(other.faceting.sort_facet_values_by_total),
            },
            pagination: PaginationAnalytics {
                max_total_hits: self.pagination.max_total_hits.or(other.pagination.max_total_hits),
            },
            stop_words: StopWordsAnalytics {
                total: self.stop_words.total.or(other.stop_words.total),
            },
            synonyms: SynonymsAnalytics { total: self.synonyms.total.or(other.synonyms.total) },
            embedders: EmbeddersAnalytics {
                total: self.embedders.total.or(other.embedders.total),
                sources: match (self.embedders.sources, other.embedders.sources) {
                    (None, None) => None,
                    (Some(sources), None) | (None, Some(sources)) => Some(sources),
                    (Some(this), Some(other)) => Some(this.union(&other).cloned().collect()),
                },
                document_template_used: match (
                    self.embedders.document_template_used,
                    other.embedders.document_template_used,
                ) {
                    (None, None) => None,
                    (Some(used), None) | (None, Some(used)) => Some(used),
                    (Some(this), Some(other)) => Some(this | other),
                },
                document_template_max_bytes: match (
                    self.embedders.document_template_max_bytes,
                    other.embedders.document_template_max_bytes,
                ) {
                    (None, None) => None,
                    (Some(bytes), None) | (None, Some(bytes)) => Some(bytes),
                    (Some(this), Some(other)) => Some(this.max(other)),
                },
                binary_quantization_used: match (
                    self.embedders.binary_quantization_used,
                    other.embedders.binary_quantization_used,
                ) {
                    (None, None) => None,
                    (Some(bq), None) | (None, Some(bq)) => Some(bq),
                    (Some(this), Some(other)) => Some(this | other),
                },
            },
            search_cutoff_ms: SearchCutoffMsAnalytics {
                search_cutoff_ms: self
                    .search_cutoff_ms
                    .search_cutoff_ms
                    .or(other.search_cutoff_ms.search_cutoff_ms),
            },
            locales: LocalesAnalytics { locales: self.locales.locales.or(other.locales.locales) },
            dictionary: DictionaryAnalytics {
                total: self.dictionary.total.or(other.dictionary.total),
            },
            separator_tokens: SeparatorTokensAnalytics {
                total: self.separator_tokens.total.or(other.non_separator_tokens.total),
            },
            non_separator_tokens: NonSeparatorTokensAnalytics {
                total: self.non_separator_tokens.total.or(other.non_separator_tokens.total),
            },
        })
    }

    fn into_event(self: Box<Self>) -> serde_json::Value {
        serde_json::to_value(*self).unwrap_or_default()
    }
}

#[derive(Serialize, Default)]
pub struct RankingRulesAnalytics {
    pub words_position: Option<usize>,
    pub typo_position: Option<usize>,
    pub proximity_position: Option<usize>,
    pub attribute_position: Option<usize>,
    pub sort_position: Option<usize>,
    pub exactness_position: Option<usize>,
    pub values: Option<String>,
}

impl RankingRulesAnalytics {
    pub fn new(rr: Option<&Vec<RankingRuleView>>) -> Self {
        RankingRulesAnalytics {
            words_position: rr.as_ref().and_then(|rr| {
                rr.iter()
                    .position(|s| matches!(s, meilisearch_types::settings::RankingRuleView::Words))
            }),
            typo_position: rr.as_ref().and_then(|rr| {
                rr.iter()
                    .position(|s| matches!(s, meilisearch_types::settings::RankingRuleView::Typo))
            }),
            proximity_position: rr.as_ref().and_then(|rr| {
                rr.iter().position(|s| {
                    matches!(s, meilisearch_types::settings::RankingRuleView::Proximity)
                })
            }),
            attribute_position: rr.as_ref().and_then(|rr| {
                rr.iter().position(|s| {
                    matches!(s, meilisearch_types::settings::RankingRuleView::Attribute)
                })
            }),
            sort_position: rr.as_ref().and_then(|rr| {
                rr.iter()
                    .position(|s| matches!(s, meilisearch_types::settings::RankingRuleView::Sort))
            }),
            exactness_position: rr.as_ref().and_then(|rr| {
                rr.iter().position(|s| {
                    matches!(s, meilisearch_types::settings::RankingRuleView::Exactness)
                })
            }),
            values: rr.as_ref().map(|rr| {
                rr.iter()
                    .filter(|s| {
                        matches!(
                            s,
                            meilisearch_types::settings::RankingRuleView::Asc(_)
                                | meilisearch_types::settings::RankingRuleView::Desc(_)
                        )
                    })
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            }),
        }
    }

    pub fn into_settings(self) -> SettingsAnalytics {
        SettingsAnalytics { ranking_rules: self, ..Default::default() }
    }
}

#[derive(Serialize, Default)]
pub struct SearchableAttributesAnalytics {
    pub total: Option<usize>,
    pub with_wildcard: Option<bool>,
}

impl SearchableAttributesAnalytics {
    pub fn new(setting: Option<&Vec<String>>) -> Self {
        Self {
            total: setting.as_ref().map(|searchable| searchable.len()),
            with_wildcard: setting
                .as_ref()
                .map(|searchable| searchable.iter().any(|searchable| searchable == "*")),
        }
    }

    pub fn into_settings(self) -> SettingsAnalytics {
        SettingsAnalytics { searchable_attributes: self, ..Default::default() }
    }
}

#[derive(Serialize, Default)]
pub struct DisplayedAttributesAnalytics {
    pub total: Option<usize>,
    pub with_wildcard: Option<bool>,
}

impl DisplayedAttributesAnalytics {
    pub fn new(displayed: Option<&Vec<String>>) -> Self {
        Self {
            total: displayed.as_ref().map(|displayed| displayed.len()),
            with_wildcard: displayed
                .as_ref()
                .map(|displayed| displayed.iter().any(|displayed| displayed == "*")),
        }
    }

    pub fn into_settings(self) -> SettingsAnalytics {
        SettingsAnalytics { displayed_attributes: self, ..Default::default() }
    }
}

#[derive(Serialize, Default)]
pub struct SortableAttributesAnalytics {
    pub total: Option<usize>,
    pub has_geo: Option<bool>,
}

impl SortableAttributesAnalytics {
    pub fn new(setting: Option<&BTreeSet<String>>) -> Self {
        Self {
            total: setting.as_ref().map(|sort| sort.len()),
            has_geo: setting.as_ref().map(|sort| sort.contains("_geo")),
        }
    }

    pub fn into_settings(self) -> SettingsAnalytics {
        SettingsAnalytics { sortable_attributes: self, ..Default::default() }
    }
}

#[derive(Serialize, Default)]
pub struct FilterableAttributesAnalytics {
    pub total: Option<usize>,
    pub has_geo: Option<bool>,
}

impl FilterableAttributesAnalytics {
    pub fn new(setting: Option<&BTreeSet<String>>) -> Self {
        Self {
            total: setting.as_ref().map(|filter| filter.len()),
            has_geo: setting.as_ref().map(|filter| filter.contains("_geo")),
        }
    }

    pub fn into_settings(self) -> SettingsAnalytics {
        SettingsAnalytics { filterable_attributes: self, ..Default::default() }
    }
}

#[derive(Serialize, Default)]
pub struct DistinctAttributeAnalytics {
    pub set: bool,
}

impl DistinctAttributeAnalytics {
    pub fn new(distinct: Option<&String>) -> Self {
        Self { set: distinct.is_some() }
    }

    pub fn into_settings(self) -> SettingsAnalytics {
        SettingsAnalytics { distinct_attribute: self, ..Default::default() }
    }
}

#[derive(Serialize, Default)]
pub struct ProximityPrecisionAnalytics {
    pub set: bool,
    pub value: Option<ProximityPrecisionView>,
}

impl ProximityPrecisionAnalytics {
    pub fn new(precision: Option<&ProximityPrecisionView>) -> Self {
        Self { set: precision.is_some(), value: precision.cloned() }
    }

    pub fn into_settings(self) -> SettingsAnalytics {
        SettingsAnalytics { proximity_precision: self, ..Default::default() }
    }
}

#[derive(Serialize, Default)]
pub struct TypoToleranceAnalytics {
    pub enabled: Option<bool>,
    pub disable_on_attributes: Option<bool>,
    pub disable_on_words: Option<bool>,
    pub min_word_size_for_one_typo: Option<u8>,
    pub min_word_size_for_two_typos: Option<u8>,
}

impl TypoToleranceAnalytics {
    pub fn new(setting: Option<&TypoSettings>) -> Self {
        Self {
            enabled: setting.as_ref().map(|s| !matches!(s.enabled, Setting::Set(false))),
            disable_on_attributes: setting
                .as_ref()
                .and_then(|s| s.disable_on_attributes.as_ref().set().map(|m| !m.is_empty())),
            disable_on_words: setting
                .as_ref()
                .and_then(|s| s.disable_on_words.as_ref().set().map(|m| !m.is_empty())),
            min_word_size_for_one_typo: setting
                .as_ref()
                .and_then(|s| s.min_word_size_for_typos.as_ref().set().map(|s| s.one_typo.set()))
                .flatten(),
            min_word_size_for_two_typos: setting
                .as_ref()
                .and_then(|s| s.min_word_size_for_typos.as_ref().set().map(|s| s.two_typos.set()))
                .flatten(),
        }
    }
    pub fn into_settings(self) -> SettingsAnalytics {
        SettingsAnalytics { typo_tolerance: self, ..Default::default() }
    }
}

#[derive(Serialize, Default)]
pub struct FacetingAnalytics {
    pub max_values_per_facet: Option<usize>,
    pub sort_facet_values_by_star_count: Option<bool>,
    pub sort_facet_values_by_total: Option<usize>,
}

impl FacetingAnalytics {
    pub fn new(setting: Option<&FacetingSettings>) -> Self {
        Self {
            max_values_per_facet: setting.as_ref().and_then(|s| s.max_values_per_facet.set()),
            sort_facet_values_by_star_count: setting.as_ref().and_then(|s| {
                s.sort_facet_values_by
                    .as_ref()
                    .set()
                    .map(|s| s.iter().any(|(k, v)| k == "*" && v == &FacetValuesSort::Count))
            }),
            sort_facet_values_by_total: setting
                .as_ref()
                .and_then(|s| s.sort_facet_values_by.as_ref().set().map(|s| s.len())),
        }
    }

    pub fn into_settings(self) -> SettingsAnalytics {
        SettingsAnalytics { faceting: self, ..Default::default() }
    }
}

#[derive(Serialize, Default)]
pub struct PaginationAnalytics {
    pub max_total_hits: Option<usize>,
}

impl PaginationAnalytics {
    pub fn new(setting: Option<&PaginationSettings>) -> Self {
        Self { max_total_hits: setting.as_ref().and_then(|s| s.max_total_hits.set()) }
    }

    pub fn into_settings(self) -> SettingsAnalytics {
        SettingsAnalytics { pagination: self, ..Default::default() }
    }
}

#[derive(Serialize, Default)]
pub struct StopWordsAnalytics {
    pub total: Option<usize>,
}

impl StopWordsAnalytics {
    pub fn new(stop_words: Option<&BTreeSet<String>>) -> Self {
        Self { total: stop_words.as_ref().map(|stop_words| stop_words.len()) }
    }

    pub fn into_settings(self) -> SettingsAnalytics {
        SettingsAnalytics { stop_words: self, ..Default::default() }
    }
}

#[derive(Serialize, Default)]
pub struct SynonymsAnalytics {
    pub total: Option<usize>,
}

impl SynonymsAnalytics {
    pub fn new(synonyms: Option<&BTreeMap<String, Vec<String>>>) -> Self {
        Self { total: synonyms.as_ref().map(|synonyms| synonyms.len()) }
    }

    pub fn into_settings(self) -> SettingsAnalytics {
        SettingsAnalytics { synonyms: self, ..Default::default() }
    }
}

#[derive(Serialize, Default)]
pub struct EmbeddersAnalytics {
    // last
    pub total: Option<usize>,
    // Merge the sources
    pub sources: Option<HashSet<String>>,
    // |=
    pub document_template_used: Option<bool>,
    // max
    pub document_template_max_bytes: Option<usize>,
    // |=
    pub binary_quantization_used: Option<bool>,
}

impl EmbeddersAnalytics {
    pub fn new(setting: Option<&BTreeMap<String, Setting<EmbeddingSettings>>>) -> Self {
        let mut sources = std::collections::HashSet::new();

        if let Some(s) = &setting {
            for source in s
                .values()
                .filter_map(|config| config.clone().set())
                .filter_map(|config| config.source.set())
            {
                use meilisearch_types::milli::vector::settings::EmbedderSource;
                match source {
                    EmbedderSource::OpenAi => sources.insert("openAi".to_string()),
                    EmbedderSource::HuggingFace => sources.insert("huggingFace".to_string()),
                    EmbedderSource::UserProvided => sources.insert("userProvided".to_string()),
                    EmbedderSource::Ollama => sources.insert("ollama".to_string()),
                    EmbedderSource::Rest => sources.insert("rest".to_string()),
                };
            }
        };

        Self {
            total: setting.as_ref().map(|s| s.len()),
            sources: Some(sources),
            document_template_used: setting.as_ref().map(|map| {
                map.values()
                    .filter_map(|config| config.clone().set())
                    .any(|config| config.document_template.set().is_some())
            }),
            document_template_max_bytes: setting.as_ref().and_then(|map| {
                map.values()
                    .filter_map(|config| config.clone().set())
                    .filter_map(|config| config.document_template_max_bytes.set())
                    .max()
            }),
            binary_quantization_used: setting.as_ref().map(|map| {
                map.values()
                    .filter_map(|config| config.clone().set())
                    .any(|config| config.binary_quantized.set().is_some())
            }),
        }
    }

    pub fn into_settings(self) -> SettingsAnalytics {
        SettingsAnalytics { embedders: self, ..Default::default() }
    }
}

#[derive(Serialize, Default)]
#[serde(transparent)]
pub struct SearchCutoffMsAnalytics {
    pub search_cutoff_ms: Option<u64>,
}

impl SearchCutoffMsAnalytics {
    pub fn new(setting: Option<&u64>) -> Self {
        Self { search_cutoff_ms: setting.copied() }
    }

    pub fn into_settings(self) -> SettingsAnalytics {
        SettingsAnalytics { search_cutoff_ms: self, ..Default::default() }
    }
}

#[derive(Serialize, Default)]
#[serde(transparent)]
pub struct LocalesAnalytics {
    pub locales: Option<BTreeSet<Locale>>,
}

impl LocalesAnalytics {
    pub fn new(rules: Option<&Vec<LocalizedAttributesRuleView>>) -> Self {
        LocalesAnalytics {
            locales: rules.as_ref().map(|rules| {
                rules
                    .iter()
                    .flat_map(|rule| rule.locales.iter().cloned())
                    .collect::<std::collections::BTreeSet<_>>()
            }),
        }
    }

    pub fn into_settings(self) -> SettingsAnalytics {
        SettingsAnalytics { locales: self, ..Default::default() }
    }
}

#[derive(Serialize, Default)]
pub struct DictionaryAnalytics {
    pub total: Option<usize>,
}

impl DictionaryAnalytics {
    pub fn new(dictionary: Option<&BTreeSet<String>>) -> Self {
        Self { total: dictionary.as_ref().map(|dictionary| dictionary.len()) }
    }

    pub fn into_settings(self) -> SettingsAnalytics {
        SettingsAnalytics { dictionary: self, ..Default::default() }
    }
}

#[derive(Serialize, Default)]
pub struct SeparatorTokensAnalytics {
    pub total: Option<usize>,
}

impl SeparatorTokensAnalytics {
    pub fn new(separator_tokens: Option<&BTreeSet<String>>) -> Self {
        Self { total: separator_tokens.as_ref().map(|separator_tokens| separator_tokens.len()) }
    }

    pub fn into_settings(self) -> SettingsAnalytics {
        SettingsAnalytics { separator_tokens: self, ..Default::default() }
    }
}

#[derive(Serialize, Default)]
pub struct NonSeparatorTokensAnalytics {
    pub total: Option<usize>,
}

impl NonSeparatorTokensAnalytics {
    pub fn new(non_separator_tokens: Option<&BTreeSet<String>>) -> Self {
        Self {
            total: non_separator_tokens
                .as_ref()
                .map(|non_separator_tokens| non_separator_tokens.len()),
        }
    }

    pub fn into_settings(self) -> SettingsAnalytics {
        SettingsAnalytics { non_separator_tokens: self, ..Default::default() }
    }
}
