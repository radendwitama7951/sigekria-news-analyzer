use handlebars::Handlebars;

pub mod app;

pub const PROJECT_SOURCE: &str = "warptest/src";

pub fn warptest_check() -> String {
    "Hello from Warptest".into()
}

#[inline]
pub fn project(path: &str) -> String {
    match path.chars().nth(0) {
        Some(c) => assert!(
            c == '/',
            "__project_fn__: ERR, Path to '{path}' must start with '/' !"
        ),
        _ => (),
    }
    format!("{PROJECT_SOURCE}{path}")
}

pub fn register_templates(hb: &mut Handlebars) {
    let templates = [
        ("index_html", "/index.html"),
        ("home_page", "/app/home/home_page.html"),
        (
            "analyze_result_component",
            "/app/home/analyze_result_component.html",
        ),
        (
            "analyze_result_error_component",
            "/app/home/analyze_result_error_component.html",
        ),
        (
            "analyze_search_component",
            "/app/home/analyze_search_component.html",
        ),
        (
            "history_drawer_component",
            "/app/home/history_drawer_component.html",
        ),
        ("auth_page", "/app/auth/auth_page.html"),
        ("login_page", "/app/auth/login_page.html"),
        ("register_page", "/app/auth/register_page.html"),
        ("error_page", "/assets/error_page.html"),
        ("phantom_html", "/assets/phantom.html"),
    ];

    for (name, path) in templates {
        hb.register_template_file(name, project(path)).unwrap();
    }
}
