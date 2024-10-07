import { createRouter, createWebHistory } from 'vue-router';
import UserLogin from '../components/UserLogin/UserLogin.vue';
import UserLogon from '../components/UserLogon/UserLogon.vue';
import DashboardView from '../components/DashboardView/DashboardView.vue';

const routes = [
    { path: '/', redirect: '/login' }, 
    { path: '/login', component: UserLogin }, 
    { path: '/logon', component: UserLogon }, 
    { path: '/dashboard', component: DashboardView }
];

const router = createRouter({
    history: createWebHistory(), 
    routes
});

export default router;
