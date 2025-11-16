<script lang="ts">
import { Menu, MenuItem, Submenu, PredefinedMenuItem  } from '@tauri-apps/api/menu';
import { onMount } from 'svelte';

onMount(async () => {
    const fileSubmenu = await Submenu.new({
        id: 'file',
        text: 'File',
        items: [
            await MenuItem.new({
                id: 'new',
                text: 'New (Ctrl+N)',
                action: () => {
                    console.log('New File pressed');
                },
            }),
            await MenuItem.new({
                id: 'open',
                text: 'Open',
                action: () => {
                    console.log('Open pressed');
                },
            }),
            await MenuItem.new({
                id: 'save',
                text: 'Save',
                action: () => {
                    console.log('Save pressed');
                },
            }),
            await MenuItem.new({
                id: 'save_as',
                text: 'Save As',
                action: () => {
                    console.log('Save As pressed');
                },
            }),
            await MenuItem.new({
                id: 'print',
                text: 'Print',
                action: () => {
                    console.log('Print pressed');
                },
            }),
            await MenuItem.new({
                id: 'exit',
                text: 'Exit',
                action: () => {
                    console.log('Exit pressed');
                },
            }),
        ],
    });

    const menu = await Menu.new({
        items: [
            fileSubmenu,
        ],
    });

    menu.setAsAppMenu().then(async (res: any) => {
        console.log('menu set success', res);
    });
});
</script>

