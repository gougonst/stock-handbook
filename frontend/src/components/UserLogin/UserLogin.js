import axios from "axios";
import emitter from '@/utils/mitt';
import { API_BASE_URL, LOGIN_API, API_TIMEOUT, LOGIN_PASSWORD_INCORRECT, LOGIN_USER_NOT_FOUND, INTERNAL_ERROR, NON_EXIST_STATUS_CODE } from "@/constants";

const loginUrl = `${API_BASE_URL}${LOGIN_API}`;

export default {
    name: 'UserLogin', 
    data() {
        return {
            username: "", 
            password: "", 
        };
    }, 
    methods: {
        async login() {
            let alertType = "";
            let alertMessage = "";

            try {
                await axios.post(loginUrl, {
                    username: this.username, 
                    password: this.password
                }, {
                    timeout: API_TIMEOUT, 
                    headers: {
                        "Content-Type": "application/json"
                    }
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
        }
    }
}
