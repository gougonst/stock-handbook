import { createApp } from 'vue';
import App from './App.vue';
import router from './router';

import HistoryView from './components/HistoryView/HistoryView.vue';
import InventoryView from './components/InventoryView/InventoryView.vue';
import UserInfoComponent from './components/UserInfoComponent/UserInfoComponent.vue';
import GlobalAlert from './components/GlobalAlert/GlobalAlert.vue';

import 'bootstrap/dist/css/bootstrap.min.css';
import "bootstrap";

const app = createApp(App);

app.component('HistoryView', HistoryView);
app.component('InventoryView', InventoryView);
app.component('UserInfoComponent', UserInfoComponent);
app.component('GlobalAlert', GlobalAlert);

app.use(router);
app.mount('#app');
