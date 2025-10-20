use yew::prelude::*;

const STYLES: &str = r#"
    .image-placeholder {
        background: linear-gradient(135deg, #f8f9fa 0%, #e9ecef 100%);
        border: 2px dashed #dee2e6;
        border-radius: 12px;
        min-height: 400px;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: all 0.3s ease;
    }
    .image-placeholder:hover {
        border-color: #adb5bd;
        background: linear-gradient(135deg, #e9ecef 0%, #dee2e6 100%);
    }
    .placeholder-content {
        text-align: center;
        color: #6c757d;
    }
    @media (max-width: 768px) {
        .image-placeholder {
            min-height: 300px;
        }
    }
"#;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <style>{STYLES}</style>
            <section class="hero is-fullheight">
                <div class="hero-body">
                    <div class="container">
                        <div class="box has-text-centered">
                            <h1 class="title is-1 has-text-primary">{"Привет, Я Инна!"}</h1>
                            <p class="subtitle is-4 has-text-grey">
                                {"Профессиональный салон визажа и ухода за бровями и ресницами в Сочи. "}
                                {"Создаю красоту, которая подчеркивает вашу индивидуальность."}
                            </p>
                        </div>

                        <div class="box has-text-centered mt-6">
                            <div class="image-placeholder">
                                <div class="placeholder-content">
                                    <span class="icon is-large">
                                        <i class="fas fa-camera fa-3x has-text-grey-light"></i>
                                    </span>
                                    <p class="has-text-grey mt-3">
                                        {"Здесь будет фотография Инны"}
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
