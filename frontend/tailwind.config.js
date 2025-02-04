export default {
    content: [
      './index.html',
      './src/**/*.{js,ts,jsx,tsx}'
    ],
    theme: {
      extend: {
        colors: {
          primary: 'rgb(124, 77, 255)',
          secondary: 'rgb(49, 27, 146)',
          foreground: 'rgb(196, 196, 196)',
          background: 'rgb(18, 18, 18)',
          'background-paper': 'rgb(30, 30, 30)',
          'background-lite': 'rgb(50, 50, 50)'
        },
        fontFamily: {
          inter: ['Inter', 'sans-serif']
        }
      }
    },
    plugins: []
  }