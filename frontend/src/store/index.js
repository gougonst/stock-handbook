import Cookies from 'js-cookie';
import { createStore } from 'vuex';

const store = createStore({
    state: {
        user: null
    }, 
    mutations: {
        setAuth(state, username) {
            console.log("Set username:", username);
            state.username = username;
        }, 
        clearAuth(state) {
            state.username = null;
        }
    }, 
    actions: {
        login({ commit }, { username }) {
            // TODO: It should add { secure: true } option after using HTTPS
            Cookies.set('username', username, { sameSite: 'strict' });
            commit('setAuth', username);
        }, 
        initializeAuth({ commit }) {
            const username = Cookies.get('username');
            console.log("Login user:", username);
            if (username) {
                commit('setAuth', username);
            } else {
                commit('clearAuth');
            }
        }
    }
});

export default store;
