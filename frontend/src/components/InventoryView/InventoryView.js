import { useStore } from "vuex";
import { ADD_INVENTORY_API, ADD_INVENTORY_ERROR, API_BASE_URL, API_TIMEOUT, DELETE_INVENTORY_API, INTERNAL_ERROR, INVENTORY_NOT_EXIST, LIST_INVENTORIES_API, LIST_INVENTORY_ERROR, NON_EXIST_STATUS_CODE } from '@/constants';
import axios from 'axios';
import { onMounted, ref } from 'vue';
import emitter from "@/utils/mitt";
import { format } from 'date-fns';

const listInventoriesUrl = `${API_BASE_URL}${LIST_INVENTORIES_API}`;
const addInventoryUrl = `${API_BASE_URL}${ADD_INVENTORY_API}`;
const deleteInventoryUrl = `${API_BASE_URL}${DELETE_INVENTORY_API}`;

export default {
    name: "InventoryView", 
    setup() {
        const store = useStore();

        const getDefaultNewItem = () => ({
            'code': '', 
            'shares': 0, 
            'buy_price': 0, 
            'principal_with_fee': '',  
            'date': '', 
            'current_price': 0
        });

        const newItem = ref(getDefaultNewItem());
        const items = ref([]);
        const totalRecord = ref(0);
        const totalPrice = ref(0);

        const calcTotalPrice = () => {
            let totalPrice = 0;
            items.value.forEach(stock => {
                totalPrice += stock.buy_price * stock.shares + calcFee(stock);
            });
            return totalPrice.toFixed();
        }

        const calcPrincipal = (stock) => {
            return stock.buy_price * stock.shares;
        }

        const calcFee = (stock) => {
            let fee = calcPrincipal(stock) * 0.001425;
            if (fee < 20) {
                fee = 20;
            }
            return fee;
        }

        const getPrincipalWithFee = (stock) => {
            return calcPrincipal(stock).toFixed() + "+" + calcFee(stock).toFixed();
        }

        const pushInventories = (inventories) => {
            items.value = [];
            for (const key in inventories) {
                console.log(`key: ${key}`);
                console.log(inventories[key]);
                let inventory = {
                    code: key, 
                    shares: inventories[key].shares, 
                    buy_price: inventories[key].buy_price.toFixed(2), 
                    date: format(inventories[key].date, "yyyy-MM-dd"), 
                    current_price: inventories[key].current_price, 
                    fee: inventories[key].fee, 
                    principal: inventories[key].principal, 
                    principal_with_fee: inventories[key].principal.toFixed() + "+" + inventories[key].fee
                }
                console.log(`inventory: ${inventory}`);
                items.value.push(inventory);
            }
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
                
                console.log("resp");
                console.log(resp);
                pushInventories(resp.data);
                console.log("items");
                console.log(items.value);
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
                await axios.post(addInventoryUrl, {
                    username: store.state.username, 
                    code: newItem.value.code, 
                    shares: parseInt(newItem.value.shares), 
                    buy_price: parseFloat(newItem.value.buy_price), 
                    date: formattedDate, 
                    current_price: parseFloat(newItem.value.current_price)
                }, {
                    timeout: API_TIMEOUT, 
                    headers: {
                        "Content-Type": "application/json"
                    }
                });

                listItems();
                newItem.value = getDefaultNewItem();
            } catch(err) {
                emitter.emit("show-alert", {
                    type: "danger", 
                    message: ADD_INVENTORY_ERROR
                });
            }
        };

        const deleteItem = async (id) => {
            let alertType = "";
            let alertMessage = "";
            try {
                await axios.post(deleteInventoryUrl, {
                    id: id
                }, {
                    timeout: API_TIMEOUT, 
                    headers: {
                        "content-Type": "application/json"
                    }
                });

                items.value = items.value.filter(item => item.id !== id);
            } catch(err) {
                if (err.response) {
                    switch (err.response.status) {
                        case 404:
                            alertType = "dander";
                            alertMessage = INVENTORY_NOT_EXIST;
                            break;
                        case 500:
                            alertType = "dander";
                            alertMessage = INTERNAL_ERROR;
                            break;
                        default:
                            alertType = "dander";
                            alertMessage = NON_EXIST_STATUS_CODE;
                            break;
                    }
                } else {
                    alertType = "dander";
                    alertMessage = INTERNAL_ERROR;
                    console.log("Frontend error: ", err);
                }

                emitter.emit("show-alert", {
                    type: alertType, 
                    message: alertMessage
                });
            }
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
            calcTotalPrice, 
            deleteItem, 
            getPrincipalWithFee
        };
    }
}