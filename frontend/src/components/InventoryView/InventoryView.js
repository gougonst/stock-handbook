import { useStore } from "vuex";
import { ADD_INVENTORY_API, ADD_INVENTORY_ERROR, API_BASE_URL, API_TIMEOUT, LIST_INVENTORIES_API, LIST_INVENTORY_ERROR } from '@/constants';
import axios from 'axios';
import { onMounted, ref } from 'vue';
import emitter from "@/utils/mitt";
import { format } from 'date-fns';

const listInventoriesUrl = `${API_BASE_URL}${LIST_INVENTORIES_API}`;
const addInventoryUrl = `${API_BASE_URL}${ADD_INVENTORY_API}`;

export default {
    name: "InventoryView", 
    setup() {
        const store = useStore();

        const getDefaultNewItem = () => ({
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

        const newItem = ref(getDefaultNewItem());
        const items = ref([]);
        const totalRecord = ref(0);
        const totalPrice = ref(0);

        const pushItem = (stock) => {
            const date = new Date(parseInt(stock.date.$date.$numberLong, 10));
            stock.date = format(date, "yyyy-MM-dd");
            items.value.push(stock);
        }

        const listItems = async () => {
            try {
                const resp = await axios.get(listInventoriesUrl, {
                    params: {
                        username: store.state.username
                    }, 
                    timeout: API_TIMEOUT, 
                    headers: {
                        "Content-Type": "application/json"
                    }
                });

                items.value = [];
                resp.data.forEach(stock => {
                    pushItem(stock);
                });
            } catch (err) {
                emitter.emit("show-alert", {
                    type: "danger", 
                    message: LIST_INVENTORY_ERROR
                });
            }
        };

        const addItem = async () => {
            try {
                const formattedDate = new Date(newItem.value.date).toISOString();
                const resp = await axios.post(addInventoryUrl, {
                    username: store.state.username, 
                    code: newItem.value.code, 
                    shares: parseInt(newItem.value.shares), 
                    buy_price: parseFloat(newItem.value.buyPrice), 
                    date: formattedDate, 
                    current_price: parseFloat(newItem.value.currentPrice)
                }, {
                    timeout: API_TIMEOUT, 
                    headers: {
                        "Content-Type": "application/json"
                    }
                });

                pushItem(resp.data);
                newItem.value = getDefaultNewItem();
            } catch(err) {
                emitter.emit("show-alert", {
                    type: "danger", 
                    message: ADD_INVENTORY_ERROR
                });
            }
        };

        const deleteItem = () => {

        };

        onMounted(() => {
            listItems();
        });

        return {
            items, 
            newItem, 
            totalRecord, 
            totalPrice, 
            addItem, 
            deleteItem
        };
    }
}