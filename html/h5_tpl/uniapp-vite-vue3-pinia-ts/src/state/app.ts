import { defineStore } from 'pinia';

interface AppState {
    conunt: number;
}

export const useAppStore = defineStore({
    id: 'app-store',
    state: (): AppState => ({
        conunt: 1,
    }),
    getters: {
        getNum(): number {
            return this.conunt;
        },
    },
    actions: {
        addnum() {
            this.conunt++;
        },
    },
});
