import { createRouter, createWebHistory } from 'vue-router';
import UserLogin from '../components/UserLogin/UserLogin.vue';
import UserLogon from '../components/UserLogon/UserLogon.vue';

const routes = [
    { path: '/', redirect: '/login' }, 
    { path: '/login', component: UserLogin }, 
    { path: '/logon', component: UserLogon }
];

const router = createRouter({
    history: createWebHistory(), 
    routes
});

export default router;
