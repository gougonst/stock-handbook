import { createApp } from 'vue';
import App from './App.vue';
import router from './router';
import UserInfoComponent from './components/UserInfoComponent/UserInfoComponent.vue';
import GlobalAlert from './components/GlobalAlert/GlobalAlert.vue';
import 'bootstrap/dist/css/bootstrap.min.css';
import "bootstrap";

const app = createApp(App);

app.component('UserInfoComponent', UserInfoComponent);
app.component('GlobalAlert', GlobalAlert);

app.use(router);
app.mount('#app');
