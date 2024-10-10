import { ref } from 'vue';

export default {
    name: 'DashboardView', 
    setup() {
        const currentTab = ref("inventory");

        const clickInventoryTab = () => {
            currentTab.value = "inventory";
        }

        const clickHistoryTab = () => {
            currentTab.value = "history";
        }

        return {
            currentTab, 
            clickInventoryTab, 
            clickHistoryTab
        };
    }
}
