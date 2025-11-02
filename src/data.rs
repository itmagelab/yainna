use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Content {
    pub hero: HeroSection,
    pub about: AboutSection,
    pub services: ServicesSection,
    pub portfolio: PortfolioSection,
    pub testimonials: TestimonialsSection,
    pub contact: ContactSection,
    pub footer: FooterSection,
    pub info: SalonInfo,
    pub contacts: Contacts,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct HeroSection {
    pub title: String,
    pub greeting: String,
    pub description: String,
    pub background_image: String,
    pub background_image_mobile: String,
    pub buttons: Vec<Button>,
    pub icon: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Button {
    pub text: String,
    pub link: String,
    #[serde(rename = "type")]
    pub button_type: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct AboutSection {
    pub title: String,
    pub subtitle: String,
    pub image: String,
    pub description: String,
    pub achievements: Vec<Achievement>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Achievement {
    pub title: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ServicesSection {
    pub title: String,
    pub subtitle: String,
    pub categories: Vec<ServiceCategory>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ServiceCategory {
    pub name: String,
    pub icon: String,
    pub description: String,
    pub items: Vec<ServiceItem>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ServiceItem {
    pub name: String,
    pub description: String,
    pub duration: String,
    pub price: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct PortfolioSection {
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub filters: Vec<String>,
    pub items: Vec<PortfolioItem>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct PortfolioItem {
    pub image: String,
    pub category: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct TestimonialsSection {
    pub title: String,
    pub subtitle: String,
    pub items: Vec<Testimonial>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Testimonial {
    pub name: String,
    pub avatar: String,
    pub rating: u8,
    pub text: String,
    pub date: String,
    pub service: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ContactSection {
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub phone: String,
    pub phone_link: String,
    pub email: String,
    pub address: String,
    pub working_hours: String,
    pub map_url: String,
    pub social: Vec<SocialLink>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct SocialLink {
    pub name: String,
    pub icon: String,
    pub link: String,
    pub color: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct FooterSection {
    pub copyright: String,
    pub description: String,
    pub links: Vec<FooterLink>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct FooterLink {
    pub text: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct SalonInfo {
    pub city: String,
    pub name: String,
    pub slogan: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Contacts {
    pub phone: String,
    pub telegram: String,
    pub whatsapp: String,
    pub instagram: String,
    pub email: String,
    pub address: String,
    pub working_hours: String,
}
