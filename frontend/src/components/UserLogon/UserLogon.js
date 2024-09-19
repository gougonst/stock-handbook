import axios from "axios";
import emitter from '@/utils/mitt';
import { API_BASE_URL, LOGON_API, API_TIMEOUT, LOGON_SUCCESS, LOGON_USER_EXIST, INTERNAL_ERROR, NON_EXIST_STATUS_CODE } from "@/constants";

const logonUrl = `${API_BASE_URL}${LOGON_API}`;

export default {
    name: 'UserLogon', 
    data() {
        return {
            username: "", 
            password: "", 
        };
    }, 
    methods: {
        async logon() {
            let alertType = "";
            let alertMessage = "";
            let linkToLogin = false;

            try {
                await axios.post(logonUrl, {
                    username: this.username, 
                    password: this.password
                }, {
                    timeout: API_TIMEOUT, 
                    headers: {
                        "Content-Type": "application/json"
                    }
                });

                alertType = "success";
                alertMessage = LOGON_SUCCESS;
                linkToLogin = true;
            } catch (err) {
                if (err.response) {
                    switch (err.response.status) {
                        case 400:
                            alertType = "warning";
                            alertMessage = LOGON_USER_EXIST;
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
            }

            emitter.emit("show-alert", {
                type: alertType, 
                message: alertMessage
            });

            if (linkToLogin) {
                this.$router.push("/login");
            }
        }
    }
}
