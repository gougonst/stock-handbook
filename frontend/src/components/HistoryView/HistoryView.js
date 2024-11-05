import { useStore } from "vuex";
import { nextTick, onMounted, ref } from 'vue';
import axios from "axios";
import { API_BASE_URL, API_TIMEOUT, LIST_SELL_HISTORY_API, LIST_SELL_ITEMS_ERROR } from "@/constants";
import emitter from "@/utils/mitt";
import { format } from 'date-fns';

const listSellItemsUrl = `${API_BASE_URL}${LIST_SELL_HISTORY_API}`;

export default {
    name: "HistoryView", 
    props: {
        transaction_type: "sell"
    }, 
    setup(props) {
        const store = useStore();

        const transaction_type = props.transaction_type;
        const items = ref([]);
        const checkNumber = ref(0);

        const selectSellItem = (item) => {
            nextTick(() => {
                if (item.checked) {
                    checkNumber.value += 1;
                } else {
                    checkNumber.value -= 1;
                }

                if (checkNumber.value < 0 || checkNumber.value >= items.value.length) {
                    let correctNumber = 0;
                    items.value.forEach(item => {
                        correctNumber += Number(item.checked);
                    });
                    checkNumber.value = correctNumber;
                }
            })
        }

        const calcPrincipal = (price, shares) => {
            return price * shares;
        }

        const calcFee = (item) => {
            let fee = calcPrincipal(item.transaction_price, item.shares) * 0.001425;
            if (fee < 20) {
                fee = 20;
            }
            return fee;
        }

        const calcTransactionTax = (item) => {
            return calcPrincipal(item.transaction_price, item.shares) * 0.003;
        }

        const calcProfitLoss = (buy_price, sell_price, shares) => {
            return calcPrincipal(sell_price, shares) - calcPrincipal(buy_price, shares);
        }

        const calcReturnRate = (buy_price, sell_price, shares) => {
            return (calcProfitLoss(buy_price, sell_price, shares) / calcPrincipal(buy_price, shares)) * 100;
        }

        const calcTotalBuyPrice = () => {
            let total = 0;
            items.value.forEach(item => {
                if (checkNumber.value == 0 || item.checked)
                    total += item.buy_price * item.shares;
            });
            return total;
        }

        const calcTotalProfitLoss = () => {
            let total = 0;
            items.value.forEach(item => {
                if (checkNumber.value == 0 || item.checked)
                    total += calcProfitLoss(item.buy_price, item.sell_price, item.shares);
            });
            return total;
        }

        const calcTotalReturnRate = () => {
            let total_profit_loss = calcTotalProfitLoss();
            let total_sell_price = 0;
            items.value.forEach(item => {
                if (checkNumber.value == 0 || item.checked)
                    total_sell_price += calcPrincipal(item.buy_price, item.shares);
            });
            return (total_profit_loss / total_sell_price) * 100;
        }

        const calcTotalCount = () => {
            if (transaction_type == "sell") return 0;
            return checkNumber.value == 0 ? items.value.length : checkNumber.value;
        }

        const pushSellItems = (sellItems) => {
            sellItems.forEach(sellItem => {
                console.log("before");
                console.log(sellItem);
                let item = {
                    'code': sellItem.code, 
                    'shares': sellItem.shares, 
                    'buy_price': transaction_type == "sell" ? sellItem.current_price : sellItem.transaction_price, 
                    'sell_price': transaction_type == "sell" ? sellItem.transaction_price : sellItem.current_price, 
                    'date': format(sellItem.date, "yyyy-MM-dd"), 
                    'fee': calcFee(sellItem), 
                    'transaction_tax': calcTransactionTax(sellItem), 
                    'profit_loss': calcProfitLoss(sellItem.current_price, sellItem.transaction_price, sellItem.shares), 
                    'return_rate': calcReturnRate(sellItem.current_price, sellItem.transaction_price, sellItem.shares), 
                    'buy_principal': calcPrincipal(sellItem.current_price, sellItem.shares), 
                    'sell_principal': calcPrincipal(sellItem.transaction_price, sellItem.shares), 
                    'checked': false, 
                };
                console.log("after");
                console.log(item);
                items.value.push(item);
            });
            console.log(items.value);
        }

        const listSellItems = async() => {
            try {

                const resp = await axios.get(listSellItemsUrl, {
                    params: {
                        username: store.state.username, 
                        transaction_type: transaction_type
                    }, 
                    timeout: API_TIMEOUT, 
                    headers: {
                        "Content-Type": "application/json"
                    }
                });

                pushSellItems(resp.data);
                console.log(items.value);
            } catch (err) {
                emitter.emit("show-alert", {
                    type: "danger", 
                    message: LIST_SELL_ITEMS_ERROR
                });
            }
        }

        onMounted(() => {
            listSellItems();
        });

        return {
            calcPrincipal, 
            calcFee, 
            calcTotalBuyPrice, 
            calcTotalCount, 
            calcTotalProfitLoss, 
            calcTotalReturnRate, 
            calcTransactionTax, 
            calcProfitLoss, 
            calcReturnRate, 
            listSellItems, 
            selectSellItem, 
            items, 
            transaction_type
        };
    }
}
