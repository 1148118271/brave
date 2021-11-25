mod route;

pub mod blog;
pub mod file;
pub mod admin;

pub use route::init;


pub mod template {
    use tera::Context;

    pub fn init(con: &mut Context) {
        con.insert("head", head());
        con.insert("footer", footer());
        con.insert("container", container());
    }

    fn head() -> &'static str {
       return
           r#"
             <title>子木 - 个人博客</title>

            <!-- meta -->
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1">

            <!-- css -->
            <link rel="stylesheet" href="/static/css/bootstrap.min_v3.css">
            <link rel="stylesheet" href="http://code.ionicframework.com/ionicons/2.0.1/css/ionicons.min.css">
            <link rel="stylesheet" href="/static/css/pace.css">
            <link rel="stylesheet" href="/static/css/custom.css">

            <!-- js -->
            <script src="/static/js/jquery-3.4.1.min.js"></script>
            <script src="/static/js/bootstrap.min_v3.js"></script>
            <script src="/static/js/pace.min.js"></script>
            <script src="/static/js/modernizr.custom.js"></script>
        "#
    }



    fn footer() -> &'static str {
        return
        r#"
            <footer id="site-footer">
                <div class="container">
                    <div class="row">
                        <div class="col-md-12">
                            <p class="copyright">&copy 2021 - 鲁ICP备2021040389</p>
                        </div>
                    </div>
                </div>
            </footer>
        "#
    }

    fn container() -> &'static str {
        return
        r#"
            <div class="container">
                <header id="site-header">
                    <div class="row">
                        <div class="col-md-4 col-sm-5 col-xs-8">
                            <div class="logo">
                                <h1><a href="/">子木</a></h1>
                            </div>
                        </div><!-- col-md-4 -->
                        <div class="col-md-8 col-sm-7 col-xs-4">
                            <nav class="main-nav" role="navigation">
                                <div class="navbar-header">
                                    <button type="button" id="trigger-overlay" class="navbar-toggle">
                                        <span class="ion-navicon"></span>
                                    </button>
                                </div>

                                <div class="collapse navbar-collapse" id="bs-example-navbar-collapse-1">
                                    <ul class="nav navbar-nav navbar-right">
                                        <li class="cl-effect-11"><a href="/" data-hover="首页">首页</a></li>
                                    </ul>
                                </div><!-- /.navbar-collapse -->
                            </nav>
                            <div id="header-search-box">
                                <a id="search-menu" href="javascript:void(0)"><span id="search-icon" class="ion-ios-search-strong"></span></a>
                                <div id="search-form" class="search-form">
                                    <form role="search" method="get" id="searchform" action="javascript:void(0)">
                                        <input type="search" placeholder="搜索" required>
                                        <button type="submit"><span class="ion-ios-search-strong"></span></button>
                                    </form>
                                </div>
                            </div>
                        </div><!-- col-md-8 -->
                    </div>
                </header>
            </div>
        "#
    }

}