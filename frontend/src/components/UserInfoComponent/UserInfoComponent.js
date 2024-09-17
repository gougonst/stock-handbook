export default {
    name: 'UserInfoComponent', 
    props: {
        username: String, 
        password: String, 
    }, 
    emits: [
        'update:username', 
        'update:password'
    ], 
    computed: {
        localUsername: {
            get() {
                return this.username;
            }, 
            set(val) {
                this.$emit("update:username", val);
            }
        }, 
        localPassword: {
            get() {
                return this.password;
            }, 
            set(val) {
                this.$emit("update:password", val);
            }
        }
    }
}
