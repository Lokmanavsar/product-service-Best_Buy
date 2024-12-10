use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Samsung 55-Inch 4K Smart TV".to_string(),
            price: 499.99,
            description: "Experience vivid colors and sharp details with this Samsung 55-inch 4K Smart TV, perfect for home entertainment.".to_string(),
            image: "/samsung_tv.jpg".to_string()
        },
        Product {
            id: 2,
            name: "Apple MacBook Pro 14-inch".to_string(),
            price: 1999.99,
            description: "Powerful performance meets sleek design in the Apple MacBook Pro, featuring the latest M2 chip.".to_string(),
            image: "/macbook_pro.jpg".to_string()
        },
        Product {
            id: 3,
            name: "Sony Noise-Canceling Headphones".to_string(),
            price: 349.99,
            description: "Immerse yourself in music with Sony's industry-leading noise-canceling headphones, offering superior sound quality.".to_string(),
            image: "/sony_headphones.jpg".to_string()
        },
        Product {
            id: 4,
            name: "Dyson V15 Cordless Vacuum".to_string(),
            price: 699.99,
            description: "Keep your home spotless with Dyson's most powerful and intelligent cordless vacuum.".to_string(),
            image: "/dyson_vacuum.jpg".to_string()
        },
        Product {
            id: 5,
            name: "Logitech MX Master 3 Mouse".to_string(),
            price: 99.99,
            description: "Boost productivity with the Logitech MX Master 3, a precision mouse designed for creatives and professionals.".to_string(),
            image: "/logitech_mouse.jpg".to_string()
        },
        Product {
            id: 6,
            name: "Instant Pot Duo 7-in-1 Cooker".to_string(),
            price: 89.99,
            description: "Simplify meal prep with the Instant Pot Duo, a versatile pressure cooker, slow cooker, and more.".to_string(),
            image: "/instant_pot.jpg".to_string()
        },
        Product {
            id: 7,
            name: "Bose Smart Soundbar 700".to_string(),
            price: 799.99,
            description: "Elevate your home theater experience with the Bose Smart Soundbar 700, offering exceptional sound and voice control.".to_string(),
            image: "/bose_soundbar.jpg".to_string()
        },
        Product {
            id: 8,
            name: "Fitbit Charge 5".to_string(),
            price: 149.99,
            description: "Track your fitness and wellness journey with the Fitbit Charge 5, featuring advanced health metrics.".to_string(),
            image: "/fitbit_charge.jpg".to_string()
        }
    ]
}
