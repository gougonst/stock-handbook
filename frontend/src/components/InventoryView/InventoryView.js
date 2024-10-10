import { ref } from 'vue';

export default {
    name: "InventoryView", 
    setup() {
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

        const items = ref([]);

        return {
            fields, 
            items
        };
    }
}