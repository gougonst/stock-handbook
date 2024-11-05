import { ref } from 'vue';

export default {
    name: 'DashboardView', 
    setup() {
        const currentTab = ref("inventory");

        const clickInventoryTab = () => {
            currentTab.value = "inventory";
        }

        const clickHistoryTab = (type) => {
            if (type == "sell")
                currentTab.value = "sellHistory";
            else
                currentTab.value = "buyHistory";
        }

        return {
            currentTab, 
            clickInventoryTab, 
            clickHistoryTab
        };
    }
}
