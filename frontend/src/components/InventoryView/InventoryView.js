import { useStore } from "vuex";
import { ADD_INVENTORY_API, ADD_INVENTORY_ERROR, API_BASE_URL, API_TIMEOUT, DELETE_INVENTORY_API, LIST_INVENTORIES_API, LIST_INVENTORY_ERROR, INTERNAL_ERROR, NON_EXIST_STATUS_CODE } from '@/constants';
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
            'transaction_price': 0, 
            'principal_with_fee': '',  
            'date': '', 
            'current_price': 0
        });

        const newItem = ref(getDefaultNewItem());
        const heldItem = ref(getDefaultNewItem());
        const sellItem = ref(getDefaultNewItem());
        const items = ref([]);
        const totalRecord = ref(0);
        const totalPrice = ref(0);
        const isSellVisible = ref(false);

        const calcTotalPrice = () => {
            let totalPrice = 0;
            items.value.forEach(inventory => {
                totalPrice += inventory.transaction_price * inventory.shares + calcFee(inventory);
            });
            return totalPrice;
        }

        const calcPrincipal = (inventory) => {
            return (inventory.transaction_price * inventory.shares);
        }

        const calcFee = (inventory) => {
            let fee = calcPrincipal(inventory) * 0.001425;
            if (fee < 20) {
                fee = 20;
            }
            return fee;
        }

        const calcTransactionTax = (inventory) => {
            return calcPrincipal(inventory) * 0.003;
        }

        const calcProfitLoss = (buyInventory, sellInventory) => {
            return calcPrincipal(sellInventory) - calcPrincipal(buyInventory);
        }

        const calcReturnRate = (buyInventory, sellInventory) => {
            return (calcProfitLoss(buyInventory, sellInventory) / calcPrincipal(buyInventory)) * 100;
        }

        const pushInventories = (inventories) => {
            items.value = [];
            for (const key in inventories) {
                console.log(`key: ${key}`);
                console.log(inventories[key]);
                let inventory = {
                    code: key, 
                    shares: inventories[key].shares, 
                    transaction_price: inventories[key].transaction_price.toFixed(2), 
                    date: format(inventories[key].date, "yyyy-MM-dd"), 
                    current_price: inventories[key].current_price, 
                    fee: inventories[key].fee, 
                    principal: inventories[key].principal.toFixed(), 
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
                pushInventories(resp.data['inventories']);
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
                    transaction_price: parseFloat(newItem.value.transaction_price), 
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

        const selectSellItem = (itemIndex) => {
            heldItem.value = items.value[itemIndex];
            sellItem.value = Object.assign({}, heldItem.value);
        }

        const deleteItem = async () => {
            let alertType = "";
            let alertMessage = "";

            try {
                let formattedDate = new Date().toISOString();
                await axios.post(deleteInventoryUrl, {
                    username: store.state.username, 
                    code: sellItem.value.code, 
                    shares: parseInt(sellItem.value.shares), 
                    transaction_price: parseFloat(sellItem.value.transaction_price), 
                    date: formattedDate, 
                    current_price: parseFloat(heldItem.value.transaction_price), 
                }, {
                    timeout: API_TIMEOUT, 
                    headers: {
                        "content-Type": "application/json"
                    }
                });

                await listItems();
            } catch(err) {
                if (err.response) {
                    switch (err.response.status) {
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
            isSellVisible, 
            items, 
            newItem, 
            sellItem, 
            heldItem, 
            totalRecord, 
            totalPrice, 
            addItem, 
            calcTotalPrice, 
            calcPrincipal, 
            calcFee, 
            calcTransactionTax, 
            calcProfitLoss, 
            calcReturnRate, 
            selectSellItem, 
            deleteItem, 
        };
    }
}