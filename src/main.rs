use data::{Button, Content};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

mod data;

#[function_component(App)]
fn app() -> Html {
    let content = use_state(|| None::<Content>);

    {
        let content = content.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                match load_content().await {
                    Ok(data) => content.set(Some(data)),
                    Err(e) => {
                        gloo_console::error!("Failed to load content:", &e);
                    }
                }
            });
            || ()
        });
    }

    match (*content).as_ref() {
        Some(data) => html! {
            <>
                <HeroSection hero={data.hero.clone()} />
                <AboutSection about={data.about.clone()} />
                <ServicesSection services={data.services.clone()} contacts={data.contacts.clone()} />
                <PortfolioSection portfolio={data.portfolio.clone()} />
                <TestimonialsSection testimonials={data.testimonials.clone()} contacts={data.contacts.clone()} />
                <ContactSection contact={data.contact.clone()} />
                <Footer footer={data.footer.clone()} contacts={data.contacts.clone()} />
            </>
        },
        None => html! {
            <div class="min-h-screen bg-gradient-to-br from-amber-50 via-yellow-50 to-orange-100 flex items-center justify-center">
                <div class="text-amber-800 text-xl">{ "Загрузка..." }</div>
            </div>
        },
    }
}

async fn load_content() -> Result<Content, String> {
    let response = Request::get("/static/content.yaml")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch content: {:?}", e))?;

    let text = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response text: {:?}", e))?;

    serde_yaml::from_str(&text).map_err(|e| format!("Failed to parse YAML: {:?}", e))
}

#[derive(Properties, PartialEq)]
struct HeroSectionProps {
    hero: data::HeroSection,
}

#[function_component(HeroSection)]
fn hero_section(props: &HeroSectionProps) -> Html {
    let hero = &props.hero;

    html! {
        <div class="relative min-h-screen flex items-center justify-center overflow-hidden" style="min-height: 100vh; min-height: -webkit-fill-available;">
            // Фоновый градиент
            <div class="absolute inset-0 bg-gradient-to-br from-amber-50 via-yellow-50 to-orange-100"></div>

            // Фоновое изображение - используем picture element для адаптивной загрузки
            <picture class="absolute inset-0 w-full h-full opacity-30 pointer-events-none" style="z-index: 0;">
                <source
                    media="(max-width: 768px)"
                    srcset={hero.background_image_mobile.clone()}
                />
                <img
                    src={hero.background_image.clone()}
                    alt=""
                    class="absolute inset-0 w-full h-full"
                    style="object-fit: cover; object-position: center; display: block;"
                    decoding="async"
                    fetchpriority="high"
                />
            </picture>

            // Затемняющий оверлей для лучшей читаемости
            <div class="absolute inset-0 bg-gradient-to-b from-amber-900/10 via-transparent to-amber-900/20" style="z-index: 1;"></div>

            // Контент поверх изображения в полупрозрачном блоке
            <div class="relative text-center px-4 py-20 max-w-4xl mx-auto" style="z-index: 10;">
                <div class="backdrop-blur-md bg-white/50 rounded-3xl shadow-2xl p-8 md:p-12 border border-amber-200/50">
                    <h1 class="text-5xl md:text-7xl font-heading font-extrabold mb-6 animate-fade-in text-amber-900">
                        { &hero.title }
                    </h1>
                    <p class="text-xl md:text-2xl font-heading font-light mb-8 text-amber-800" style="font-weight: 300;">
                        { &hero.greeting }
                    </p>
                    <p class="text-lg md:text-xl font-body font-light mb-8 text-amber-700 max-w-2xl mx-auto" style="font-weight: 300;">
                        { &hero.description }
                    </p>
                    <div class="flex flex-col sm:flex-row gap-4 justify-center mt-10">
                        { for hero.buttons.iter().map(|btn| render_button(btn)) }
                    </div>
                    <div class="mt-12">
                        <i class={format!("fas {} text-5xl text-amber-600 opacity-70 animate-pulse", hero.icon)}></i>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn render_button(button: &Button) -> Html {
    let class = if button.button_type == "primary" {
        "bg-amber-600 text-white px-8 py-3 rounded-full font-body font-semibold hover:bg-amber-700 transition-all duration-300 shadow-lg"
    } else {
        "bg-transparent border-2 border-amber-600 text-amber-700 px-8 py-3 rounded-full font-body font-semibold hover:bg-amber-600 hover:text-white transition-all duration-300"
    };

    html! {
        <a href={button.link.clone()} class={class}>
            { &button.text }
        </a>
    }
}

#[derive(Properties, PartialEq)]
struct AboutSectionProps {
    about: data::AboutSection,
}

#[function_component(AboutSection)]
fn about_section(props: &AboutSectionProps) -> Html {
    let about = &props.about;

    html! {
        <section class="py-20 px-4 bg-white">
            <div class="max-w-6xl mx-auto">
                <div class="text-center mb-12">
                    <h2 class="text-4xl md:text-5xl font-heading font-extrabold text-amber-900 mb-3">
                        { &about.title }
                    </h2>
                    <p class="text-lg text-amber-700 font-body">
                        { &about.subtitle }
                    </p>
                </div>

                <div class="grid md:grid-cols-2 gap-12 items-center mb-16">
                    <div class="order-2 md:order-1">
                        <div class="prose prose-lg">
                            { about.description.split("\n\n").map(|paragraph| {
                                html! {
                                    <p class="text-amber-800 font-body mb-4 leading-relaxed">
                                        { paragraph.trim() }
                                    </p>
                                }
                            }).collect::<Html>() }
                        </div>
                    </div>

                    <div class="order-1 md:order-2">
                        <div class="relative">
                            <div class="aspect-square rounded-2xl overflow-hidden shadow-2xl bg-gradient-to-br from-amber-100 to-orange-200">
                                <img
                                    src={about.image.clone()}
                                    alt={format!("{} - {}", &about.title, &about.subtitle)}
                                    class="w-full h-full object-cover"
                                />
                            </div>
                        </div>
                    </div>
                </div>

                <div class="grid md:grid-cols-3 gap-8">
                    { for about.achievements.iter().map(|achievement| {
                        html! {
                            <div class="text-center p-6 rounded-xl bg-amber-50 hover:bg-amber-100 transition-all duration-300">
                                <div class="mb-4">
                                    <i class={format!("fas {} text-4xl text-amber-600", achievement.icon)}></i>
                                </div>
                                <h3 class="text-xl font-heading font-bold text-amber-900 mb-2">
                                    { &achievement.title }
                                </h3>
                                <p class="text-amber-700 font-body">
                                    { &achievement.description }
                                </p>
                            </div>
                        }
                    }) }
                </div>
            </div>
        </section>
    }
}

#[derive(Properties, PartialEq)]
struct ServicesSectionProps {
    services: data::ServicesSection,
    contacts: data::Contacts,
}

#[function_component(ServicesSection)]
fn services_section(props: &ServicesSectionProps) -> Html {
    let services = &props.services;

    html! {
        <section class="py-20 px-4 bg-gradient-to-br from-amber-50 via-yellow-50 to-orange-100">
            <div class="max-w-7xl mx-auto">
                <div class="text-center mb-16">
                    <h2 class="text-4xl md:text-5xl font-heading font-extrabold text-amber-900 mb-3">
                        { &services.title }
                    </h2>
                    <p class="text-lg text-amber-700 font-body">
                        { &services.subtitle }
                    </p>
                </div>

                <div class="grid md:grid-cols-3 gap-8">
                    { for services.categories.iter().map(|category| {
                        html! {
                            <div class="bg-white rounded-2xl shadow-xl overflow-hidden hover:shadow-2xl transition-all duration-300 transform hover:-translate-y-2">
                                <div class="bg-gradient-to-r from-amber-500 to-orange-500 p-6 text-center">
                                    <div class="mb-3">
                                        <i class={format!("fas {} text-5xl text-white", category.icon)}></i>
                                    </div>
                                    <h3 class="text-2xl font-heading font-bold text-white mb-2">
                                        { &category.name }
                                    </h3>
                                    <p class="text-white/90 font-body text-sm">
                                        { &category.description }
                                    </p>
                                </div>

                                <div class="p-6">
                                    <div class="space-y-4">
                                        { for category.items.iter().map(|item| {
                                            html! {
                                                <div class="border-b border-amber-100 pb-4 last:border-0">
                                                    <div class="flex justify-between items-start mb-2">
                                                        <h4 class="font-heading font-semibold text-amber-900 text-lg">
                                                            { &item.name }
                                                        </h4>
                                                        <span class="font-heading font-bold text-amber-600 text-lg whitespace-nowrap ml-2">
                                                            { &item.price }
                                                        </span>
                                                    </div>
                                                    <p class="text-sm text-amber-700 font-body mb-1">
                                                        { &item.description }
                                                    </p>
                                                    <div class="flex items-center text-xs text-amber-600 font-body">
                                                        <i class="far fa-clock mr-1"></i>
                                                        { &item.duration }
                                                    </div>
                                                </div>
                                            }
                                        }) }
                                    </div>

                                    <div class="mt-6">
                                        <a
                                            href={props.contacts.telegram.clone()}
                                            class="block w-full bg-amber-600 text-white text-center py-3 rounded-full font-body font-semibold hover:bg-amber-700 transition-all duration-300"
                                        >
                                            { "Записаться" }
                                        </a>
                                    </div>
                                </div>
                            </div>
                        }
                    }) }
                </div>
            </div>
        </section>
    }
}

#[derive(Properties, PartialEq)]
struct PortfolioSectionProps {
    portfolio: data::PortfolioSection,
}

#[function_component(PortfolioSection)]
fn portfolio_section(props: &PortfolioSectionProps) -> Html {
    let portfolio = &props.portfolio;
    let selected_filter = use_state(|| "Все".to_string());

    let filtered_items = if *selected_filter == "Все" {
        portfolio.items.clone()
    } else {
        portfolio
            .items
            .iter()
            .filter(|item| item.category == *selected_filter)
            .cloned()
            .collect::<Vec<_>>()
    };

    html! {
        <section class="py-20 px-4 bg-white">
            <div class="max-w-7xl mx-auto">
                <div class="text-center mb-12">
                    <h2 class="text-4xl md:text-5xl font-heading font-extrabold text-amber-900 mb-3">
                        { &portfolio.title }
                    </h2>
                    <p class="text-xl text-amber-700 font-body mb-2">
                        { &portfolio.subtitle }
                    </p>
                    <p class="text-md text-amber-600 font-body">
                        { &portfolio.description }
                    </p>
                </div>

                // Фильтры
                <div class="flex flex-wrap justify-center gap-3 mb-12">
                    { for portfolio.filters.iter().map(|filter| {
                        let filter_clone = filter.clone();
                        let selected_filter_clone = selected_filter.clone();
                        let is_active = *selected_filter == *filter;

                        let onclick = Callback::from(move |_| {
                            selected_filter_clone.set(filter_clone.clone());
                        });

                        html! {
                            <button
                                onclick={onclick}
                                class={format!(
                                    "px-6 py-2 rounded-full font-body font-semibold transition-all duration-300 {}",
                                    if is_active {
                                        "bg-amber-600 text-white shadow-lg"
                                    } else {
                                        "bg-amber-100 text-amber-700 hover:bg-amber-200"
                                    }
                                )}
                            >
                                { filter }
                            </button>
                        }
                    }) }
                </div>

                // Галерея
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
                    { for filtered_items.iter().map(|item| {
                        html! {
                            <div class="group relative overflow-hidden rounded-2xl shadow-lg hover:shadow-2xl transition-all duration-300 transform hover:-translate-y-2 bg-gradient-to-br from-amber-100 to-orange-200">
                                <div class="aspect-square overflow-hidden">
                                    <img
                                        src={item.image.clone()}
                                        alt={item.title.clone()}
                                        class="w-full h-full object-cover group-hover:scale-110 transition-transform duration-500"
                                    />
                                </div>

                                // Оверлей с информацией
                                <div class="absolute inset-0 bg-gradient-to-t from-amber-900/90 via-amber-900/50 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300 flex flex-col justify-end p-6">
                                    <span class="text-amber-300 text-sm font-body mb-1">
                                        { &item.category }
                                    </span>
                                    <h3 class="text-white text-xl font-heading font-bold mb-2">
                                        { &item.title }
                                    </h3>
                                    <p class="text-white/90 text-sm font-body">
                                        { &item.description }
                                    </p>
                                </div>
                            </div>
                        }
                    }) }
                </div>

                { if filtered_items.is_empty() {
                    html! {
                        <div class="text-center py-12">
                            <p class="text-amber-600 text-lg font-body">
                                { "Работы не найдены" }
                            </p>
                        </div>
                    }
                } else {
                    html! {}
                }}
            </div>
        </section>
    }
}

#[derive(Properties, PartialEq)]
struct TestimonialsSectionProps {
    testimonials: data::TestimonialsSection,
    contacts: data::Contacts,
}

#[function_component(TestimonialsSection)]
fn testimonials_section(props: &TestimonialsSectionProps) -> Html {
    let testimonials = &props.testimonials;

    html! {
        <section class="py-20 px-4 bg-gradient-to-br from-amber-50 via-yellow-50 to-orange-100">
            <div class="max-w-7xl mx-auto">
                <div class="text-center mb-16">
                    <h2 class="text-4xl md:text-5xl font-heading font-extrabold text-amber-900 mb-3">
                        { &testimonials.title }
                    </h2>
                    <p class="text-lg text-amber-700 font-body">
                        { &testimonials.subtitle }
                    </p>
                </div>

                <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                    { for testimonials.items.iter().map(|testimonial| {
                        html! {
                            <div class="bg-white rounded-2xl shadow-lg hover:shadow-2xl transition-all duration-300 p-6 transform hover:-translate-y-2">
                                // Аватар и имя
                                <div class="flex items-center mb-4">
                                    <div class="w-14 h-14 rounded-full bg-gradient-to-br from-amber-400 to-orange-500 flex items-center justify-center text-white font-heading font-bold text-lg mr-4">
                                        { &testimonial.avatar }
                                    </div>
                                    <div>
                                        <h3 class="font-heading font-bold text-amber-900 text-lg">
                                            { &testimonial.name }
                                        </h3>
                                        <p class="text-sm text-amber-600 font-body">
                                            { &testimonial.service }
                                        </p>
                                    </div>
                                </div>

                                // Рейтинг
                                <div class="flex mb-3">
                                    { for (0..testimonial.rating).map(|_| {
                                        html! {
                                            <i class="fas fa-star text-amber-500"></i>
                                        }
                                    }) }
                                </div>

                                // Текст отзыва
                                <div class="relative mb-4">
                                    <i class="fas fa-quote-left text-3xl text-amber-200 absolute -top-2 -left-1"></i>
                                    <p class="text-amber-800 font-body leading-relaxed pl-8 pr-4">
                                        { &testimonial.text }
                                    </p>
                                    <i class="fas fa-quote-right text-3xl text-amber-200 absolute -bottom-2 right-0"></i>
                                </div>

                                // Дата
                                <div class="text-sm text-amber-600 font-body text-right">
                                    { &testimonial.date }
                                </div>
                            </div>
                        }
                    }) }
                </div>

                // Ссылка на больше отзывов
                <div class="text-center mt-12">
                    <p class="text-amber-700 font-body mb-4">
                        { "Больше отзывов в моих социальных сетях" }
                    </p>
                    <div class="flex justify-center gap-4">
                        <a href={props.contacts.instagram.clone()} target="_blank" class="text-amber-600 hover:text-amber-700 transition-colors duration-300">
                            <i class="fab fa-instagram text-3xl"></i>
                        </a>
                        <a href={props.contacts.telegram.clone()} target="_blank" class="text-amber-600 hover:text-amber-700 transition-colors duration-300">
                            <i class="fab fa-telegram text-3xl"></i>
                        </a>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[derive(Properties, PartialEq)]
struct ContactSectionProps {
    contact: data::ContactSection,
}

#[function_component(ContactSection)]
fn contact_section(props: &ContactSectionProps) -> Html {
    let contact = &props.contact;

    html! {
        <section class="py-20 px-4 bg-white">
            <div class="max-w-7xl mx-auto">
                <div class="text-center mb-16">
                    <h2 class="text-4xl md:text-5xl font-heading font-extrabold text-amber-900 mb-3">
                        { &contact.title }
                    </h2>
                    <p class="text-xl text-amber-700 font-body mb-2">
                        { &contact.subtitle }
                    </p>
                    <p class="text-md text-amber-600 font-body">
                        { &contact.description }
                    </p>
                </div>

                <div class="grid md:grid-cols-2 gap-12">
                    // Контактная информация
                    <div class="space-y-6">
                        <div class="bg-gradient-to-br from-amber-50 to-orange-100 rounded-2xl p-8 shadow-lg">
                            <h3 class="text-2xl font-heading font-bold text-amber-900 mb-6">
                                { "Контактная информация" }
                            </h3>

                            // Телефон
                            <div class="flex items-start mb-4">
                                <div class="w-12 h-12 bg-amber-600 rounded-full flex items-center justify-center text-white mr-4 flex-shrink-0">
                                    <i class="fas fa-phone-alt"></i>
                                </div>
                                <div>
                                    <p class="text-sm text-amber-600 font-body mb-1">{ "Телефон" }</p>
                                    <a href={contact.phone_link.clone()} class="text-lg font-heading font-semibold text-amber-900 hover:text-amber-700 transition-colors">
                                        { &contact.phone }
                                    </a>
                                </div>
                            </div>

                            // Email
                            <div class="flex items-start mb-4">
                                <div class="w-12 h-12 bg-amber-600 rounded-full flex items-center justify-center text-white mr-4 flex-shrink-0">
                                    <i class="fas fa-envelope"></i>
                                </div>
                                <div>
                                    <p class="text-sm text-amber-600 font-body mb-1">{ "Email" }</p>
                                    <a href={format!("mailto:{}", contact.email)} class="text-lg font-heading font-semibold text-amber-900 hover:text-amber-700 transition-colors">
                                        { &contact.email }
                                    </a>
                                </div>
                            </div>

                            // Адрес
                            <div class="flex items-start mb-4">
                                <div class="w-12 h-12 bg-amber-600 rounded-full flex items-center justify-center text-white mr-4 flex-shrink-0">
                                    <i class="fas fa-map-marker-alt"></i>
                                </div>
                                <div>
                                    <p class="text-sm text-amber-600 font-body mb-1">{ "Адрес" }</p>
                                    <p class="text-lg font-body text-amber-900">
                                        { &contact.address }
                                    </p>
                                </div>
                            </div>

                            // Часы работы
                            <div class="flex items-start">
                                <div class="w-12 h-12 bg-amber-600 rounded-full flex items-center justify-center text-white mr-4 flex-shrink-0">
                                    <i class="fas fa-clock"></i>
                                </div>
                                <div>
                                    <p class="text-sm text-amber-600 font-body mb-1">{ "Часы работы" }</p>
                                    <p class="text-lg font-body text-amber-900">
                                        { &contact.working_hours }
                                    </p>
                                </div>
                            </div>
                        </div>

                        // Социальные сети
                        <div class="bg-gradient-to-br from-amber-50 to-orange-100 rounded-2xl p-8 shadow-lg">
                            <h3 class="text-2xl font-heading font-bold text-amber-900 mb-6">
                                { "Мы в социальных сетях" }
                            </h3>
                            <div class="grid grid-cols-2 gap-4">
                                { for contact.social.iter().map(|social| {
                                    html! {
                                        <a
                                            href={social.link.clone()}
                                            target="_blank"
                                            class={format!("flex items-center justify-center bg-white rounded-xl p-4 shadow hover:shadow-lg transition-all duration-300 transform hover:-translate-y-1 text-amber-600 {}", social.color)}
                                        >
                                            <i class={format!("fab {} text-3xl mr-3", social.icon)}></i>
                                            <span class="font-body font-semibold">{ &social.name }</span>
                                        </a>
                                    }
                                }) }
                            </div>
                        </div>
                    </div>

                    // Карта
                    <div class="bg-gradient-to-br from-amber-50 to-orange-100 rounded-2xl p-2 shadow-lg h-full min-h-[500px]">
                        <div class="w-full h-full rounded-xl overflow-hidden">
                            <iframe
                                src={contact.map_url.clone()}
                                width="100%"
                                height="100%"
                                style="border:0; min-height: 480px;"
                                allowfullscreen={true}
                                loading="lazy"
                                referrerpolicy="no-referrer-when-downgrade"
                            ></iframe>
                        </div>
                    </div>
                </div>

                // Кнопка "Наверх"
                <div class="text-center mt-12">
                    <button
                        onclick={Callback::from(|_| {
                            web_sys::window()
                                .and_then(|w| w.document())
                                .map(|d| {
                                    d.document_element()
                                        .map(|e| e.scroll_to_with_x_and_y(0.0, 0.0))
                                });
                        })}
                        class="bg-amber-600 text-white px-8 py-3 rounded-full font-body font-semibold hover:bg-amber-700 transition-all duration-300 shadow-lg inline-flex items-center"
                    >
                        <i class="fas fa-arrow-up mr-2"></i>
                        { "Наверх" }
                    </button>
                </div>
            </div>
        </section>
    }
}

#[derive(Properties, PartialEq)]
struct FooterProps {
    footer: data::FooterSection,
    contacts: data::Contacts,
}

#[function_component(Footer)]
fn footer(props: &FooterProps) -> Html {
    let footer = &props.footer;

    html! {
        <footer class="bg-gradient-to-r from-amber-900 to-orange-900 text-white py-12 px-4">
            <div class="max-w-7xl mx-auto">
                <div class="grid md:grid-cols-3 gap-8 mb-8">
                    // О студии
                    <div>
                        <h3 class="text-2xl font-heading font-bold mb-4">
                            { "Я Инна" }
                        </h3>
                        <p class="text-white/80 font-body leading-relaxed">
                            { &footer.description }
                        </p>
                    </div>

                    // Быстрые ссылки
                    <div>
                        <h3 class="text-xl font-heading font-bold mb-4">
                            { "Информация" }
                        </h3>
                        <ul class="space-y-2">
                            { for footer.links.iter().map(|link| {
                                html! {
                                    <li>
                                        <a href={link.url.clone()} class="text-white/80 hover:text-white transition-colors font-body">
                                            { &link.text }
                                        </a>
                                    </li>
                                }
                            }) }
                        </ul>
                    </div>

                    // Студия
                    <div>
                        <h3 class="text-xl font-heading font-bold mb-4">
                            { "Студия красоты" }
                        </h3>
                        <p class="text-white/80 font-body mb-4">
                            { "Сочи, Хостинский район" }
                        </p>
                        <div class="flex gap-4">
                            <a href={props.contacts.instagram.clone()} target="_blank" class="text-white/80 hover:text-white transition-colors">
                                <i class="fab fa-instagram text-2xl"></i>
                            </a>
                            <a href={props.contacts.telegram.clone()} target="_blank" class="text-white/80 hover:text-white transition-colors">
                                <i class="fab fa-telegram text-2xl"></i>
                            </a>
                            <a href={props.contacts.whatsapp.clone()} target="_blank" class="text-white/80 hover:text-white transition-colors">
                                <i class="fab fa-whatsapp text-2xl"></i>
                            </a>
                            <a href={props.contacts.vk.clone()} target="_blank" class="text-white/80 hover:text-white transition-colors">
                                <i class="fab fa-vk text-2xl"></i>
                            </a>
                        </div>
                    </div>
                </div>

                // Копирайт
                <div class="border-t border-white/20 pt-8 text-center">
                    <p class="text-white/60 font-body">
                        { &footer.copyright }
                    </p>
                </div>
            </div>
        </footer>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
