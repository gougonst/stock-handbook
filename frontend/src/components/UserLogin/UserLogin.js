import axios from "axios";
import emitter from '@/utils/mitt';
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { useStore } from "vuex";
import { API_BASE_URL, LOGIN_API, API_TIMEOUT, LOGIN_PASSWORD_INCORRECT, LOGIN_USER_NOT_FOUND, INTERNAL_ERROR, NON_EXIST_STATUS_CODE } from "@/constants";

const loginUrl = `${API_BASE_URL}${LOGIN_API}`;

export default {
    name: 'UserLogin', 
    setup() {
        const router = useRouter();
        const store = useStore();

        const username = ref(""); 
        const password = ref("");

        const login = async () => {
            let alertType = "";
            let alertMessage = "";

            try {
                await axios.post(loginUrl, {
                    username: username.value, 
                    password: password.value
                }, {
                    timeout: API_TIMEOUT, 
                    headers: {
                        "Content-Type": "application/json"
                    }
                });

                // Store username to global state
                store.dispatch('login', { username: username.value });

                // Go to dashboard page
                router.push({
                    path: 'dashboard'
                });
            } catch (err) {
                if (err.response) {
                    switch (err.response.status) {
                        case 400:
                            alertType = "warning";
                            alertMessage = LOGIN_PASSWORD_INCORRECT;
                            break;
                        case 404:
                            alertType = "warning";
                            alertMessage = LOGIN_USER_NOT_FOUND;
                            break;
                        case 500:
                            alertType = "danger";
                            alertMessage = INTERNAL_ERROR;
                            break;
                        default:
                            alertType = "danger";
                            alertMessage = NON_EXIST_STATUS_CODE;
                            break;
                    } 
                } else {
                    alertType = "danger";
                    alertMessage = INTERNAL_ERROR;
                    console.log("Frontend error: ", err);
                }

                emitter.emit("show-alert", {
                    type: alertType, 
                    message: alertMessage
                });
            }
        };

        return {
            username, 
            password, 
            login
        };
    }
}
