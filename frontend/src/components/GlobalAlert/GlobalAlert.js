import { onMounted, onUnmounted, ref } from 'vue';
import emitter from "@/utils/mitt";

export default {
    name: "GlobalAlert", 
    setup() {
        const visible = ref(false);
        const alertType = ref(null);
        const alertMessage = ref(null);

        const showAlert = ({type, message}) => {
            visible.value = true;
            alertType.value = type;
            alertMessage.value = message;
            setTimeout(() => {
                visible.value = false;
            }, 3000);
        };

        onMounted(() => {
            emitter.on("show-alert", showAlert);
        });

        onUnmounted(() => {
            emitter.off("show-alert", showAlert);
        });

        return {
            visible, 
            alertType, 
            alertMessage
        };
    }
}
