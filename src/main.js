import { createApp } from "vue";
// import "./styles.css";
import App from "./App.vue";

import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'

const app = createApp(App)

//还有这里
app.use(ElementPlus)

createApp(App).mount('#app')
