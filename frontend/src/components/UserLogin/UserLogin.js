import axios from "axios"
import { API_BASE_URL, LOGIN_API, API_TIMEOUT } from "@/constants"

const login_url = `${API_BASE_URL}${LOGIN_API}`;

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
            try {
                console.log(login_url);
                const resp = await axios.post(login_url, {
                    username: this.username, 
                    password: this.password
                }, {
                    timeout: API_TIMEOUT, 
                    headers: {
                        "Content-Type": "application/json"
                    }
                });
                console.log("Response: ", resp);
            } catch (err) {
                console.error("Login error: ", err);
            }
        }
    }
}
