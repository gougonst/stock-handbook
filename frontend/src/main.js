import { createApp } from 'vue';
import App from './App.vue';
import ElementPlus from 'element-plus';
import router from './router';
import store from './store';

import HistoryView from './components/HistoryView/HistoryView.vue';
import InventoryView from './components/InventoryView/InventoryView.vue';
import UserInfoComponent from './components/UserInfoComponent/UserInfoComponent.vue';
import GlobalAlert from './components/GlobalAlert/GlobalAlert.vue';

import 'bootstrap/dist/css/bootstrap.min.css';
import "bootstrap";
import "element-plus/dist/index.css";

const app = createApp(App);

app.component('HistoryView', HistoryView);
app.component('InventoryView', InventoryView);
app.component('UserInfoComponent', UserInfoComponent);
app.component('GlobalAlert', GlobalAlert);

app.use(ElementPlus);
app.use(router);
app.use(store);
app.mount('#app');

store.dispatch('initializeAuth');
