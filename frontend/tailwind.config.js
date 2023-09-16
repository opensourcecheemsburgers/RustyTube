/** @type {import('tailwindcss').Config} */
module.exports = {
    mode: 'jit',
    content: ['./css/*.{js,ts,jsx,tsx,css,scss,html}', './css/**/*.{js,ts,jsx,tsx}',
        './src/**/*.{js,ts,jsx,tsx,rs,scss,css,html}', './index.html',  './src/main.rs'
    ],
    plugins: [
        require('daisyui'),
        require('@tailwindcss/typography'),
        require("tailwindcss-animate")
    ],
    theme: {
        fontFamily: {
            'sans': ['Open Sans', 'Noto Color Emoji'],
            'display': ['Comfortaa', 'Noto Color Emoji'],
            'mono': ['Fira Mono', 'Noto Color Emoji']
        },
    },
    daisyui: {
        themes: ["light", "dark", "cupcake",
            "bumblebee", "emerald", "corporate",
            "synthwave", "retro", "cyberpunk",
            "valentine", "halloween", "garden",
            "forest", "aqua", "lofi",
            "pastel", "fantasy", "wireframe",
            "black", "luxury", "dracula",
            "cmyk", "autumn", "business",
            "acid", "lemonade", "night",
            "coffee", "winter", {
                ubiquity: {
                    "primary": "#38BDF8",
                    "secondary": "#818CF8",
                    "accent": "#F471B5",
                    "neutral": "#1E293B",
                    "base-100": "#0F172A",
                    "info": "#0CA5E9",
                    "success": "#2DD4BF",
                    "warning": "#F4BF50",
                    "error": "#FB7085",
                },
            }
        ],
    }
}