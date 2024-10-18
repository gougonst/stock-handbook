import { useStore } from "vuex";
import { API_BASE_URL, API_TIMEOUT, LIST_INVENTORIES_API } from '@/constants';
import axios from 'axios';
import { onMounted, ref } from 'vue';

const listInventoriesUrl = `${API_BASE_URL}${LIST_INVENTORIES_API}`;

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
            try {
                console.log(store.state.username);
                const resp = await axios.get(listInventoriesUrl, {
                    params: {
                        username: store.state.username
                    }, 
                    timeout: API_TIMEOUT, 
                    headers: {
                        "Content-Type": "application/json"
                    }
                });

                const stocks = resp.data;
                console.log(stocks);
            } catch (err) {
                // pass
            }
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