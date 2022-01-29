use super::common::{footer::Footer, header::Header};
use yew::{function_component, html, Html};

#[function_component(Index)]
pub fn index() -> Html {
    html! {
        <>
            <Header />
            <>
            <nav>
              <div class="nav-wrapper">
                  <div class="col s12">
                  <a href="#!" class="breadcrumb">{"不学网"}</a>
                  <a href="#!" class="breadcrumb">{"用户"}</a>
                  <a href="#!" class="breadcrumb">{"登录"}</a>
                  </div>
              </div>
              </nav>
            <div class="row s8">
                <form class="col s8 offset4">
                <div class="row">
                    <div class="input-field col s12">
                    <input placeholder="请输入用户名" id="first_name" type="text" class="validate" />
                    <label for="first_name">{"用户名"}</label>
                    </div>
                </div>
                <div class="row">
                    <div class="input-field col s12">
                    <input id="password" type="password" class="validate" />
                    <label for="password">{"密码"}</label>
                    </div>
                </div>
                <div class="row">
                    <div class="col s12">
                        <div class="input-field inline">
                            <button class="waves-effect waves-light btn-large">{"登录"}</button>
                        </div>
                    </div>
                </div>
                </form>
            </div>
            </>
            <Footer />
        </>
    }
}
