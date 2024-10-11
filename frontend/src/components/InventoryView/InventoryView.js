import { useStore } from "vuex";
import { onMounted, ref } from 'vue';

export default {
    name: "InventoryView", 
    setup() {
        const store = useStore();

        const fields = ref([
            { key: 'code', label: "代碼" },
            { key: 'shares', label: "股數" },
            { key: 'buyPrice', label: "買價" },
            { key: 'principal', label: "本金" },
            { key: 'date', label: "日期" },
            { key: 'currentPrice', label: "現價" },
            { key: 'profitLoss', label: "損益" },
            { key: 'returnRate', label: "報酬率" },
            { key: 'balance', label: "平衡" }
        ]);
        const newItem = ref({
            'code': '', 
            'shares': 0, 
            'buyPrice': 0, 
            'principal': '', 
            'date': '', 
            'currentPrice': 0, 
            'profitLoss': 0, 
            'returnRate': 0, 
            'balance': 0
        });
        const totalRecord = ref(0);
        const totalPrice = ref(0);

        const listItems = async () => {

        };

        const addItem = () => {
            
        };

        const deleteItem = () => {

        };

        onMounted(() => {
            listItems();
        });

        return {
            fields, 
            newItem, 
            totalRecord, 
            totalPrice, 
            addItem, 
            deleteItem
        };
    }
}