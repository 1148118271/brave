use tera::Context;

pub fn head() -> Context {
    let head =
    r#"
        <link rel="stylesheet" type="text/css" href="/static/css/materialdesignicons.min.css">
        <link rel="stylesheet" type="text/css" href="/static/css/bootstrap.min.css">
        <link rel="stylesheet" type="text/css" href="/static/css/bootstrap-table.min.css">
        <link rel="stylesheet" type="text/css" href="/static/css/multitabs.min.css">
        <link rel="stylesheet" type="text/css" href="/static/css/animate.min.css">
        <link rel="stylesheet" type="text/css" href="/static/css/style.min.css">


        <script type="text/javascript" src="/static/js/jquery-3.4.1.min.js"></script>
        <script type="text/javascript" src="/static/js/popper.min.js"></script>
        <script type="text/javascript" src="/static/js/bootstrap.min.js"></script>
        <script type="text/javascript" src="/static/js/bootstrap-table.min.js"></script>
        <script type="text/javascript" src="/static/js/bootstrap-table-zh-CN.js"></script>
        <script type="text/javascript" src="/static/js/perfect-scrollbar.min.js"></script>
        <script type="text/javascript" src="/static/js/multitabs.min.js"></script>
        <script type="text/javascript" src="/static/js/jquery.cookie.min.js"></script>
        <script type="text/javascript" src="/static/js/index.min.js"></script>
        <script type="text/javascript" src="/static/js/main.min.js"></script>
        <script type="text/javascript" src="/static/js/layer.js"></script>

    "#;

    let mut context = Context::new();
    context.insert("head", head);
    context
}