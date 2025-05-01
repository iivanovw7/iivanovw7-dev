use crate::{
    types::config::{Breadcrumb, Breadcrumbs, BreadcrumbsConfig},
    utils::string::capitalize,
};

pub fn generate_breadcrumbs(config: BreadcrumbsConfig) -> Breadcrumbs {
    let path = config.path;

    let segments: Vec<&str> = path
        .split("/")
        .filter(|segment| !segment.is_empty())
        .collect();

    let mut crumbs: Vec<Breadcrumb> = Vec::new();
    let mut current_path: String = String::new();

    crumbs.push(Breadcrumb {
        label: "Home".into(),
        path: "/".into(),
    });

    for (index, segment) in segments.iter().enumerate() {
        if index < segments.len() - 1 {
            current_path.push('/');
            current_path.push_str(segment);

            let label = segment.replace("-", " ").to_string();
            let path = current_path.to_string();

            crumbs.push(Breadcrumb {
                label: capitalize(&label),
                path,
            })
        }
    }

    let mut leaf = segments
        .last()
        .map(|segment| segment.replace("-", " "))
        .unwrap_or_default();

    if let Some(custom_leaf) = config.leaf {
        leaf = custom_leaf.clone();
    }

    Breadcrumbs {
        crumbs,
        leaf: capitalize(&leaf),
    }
}
