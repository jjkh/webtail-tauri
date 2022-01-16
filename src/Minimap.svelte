<script lang="ts">
    import { onMount } from "svelte";

    let minimap: HTMLCanvasElement;
    let scrollY: number;
    let context: CanvasRenderingContext2D;
    let content: HTMLDivElement;

    $: scrollY && repaint();

    function repaint() {
        requestAnimationFrame(() => {
            console.log(`scrolled to ${scrollY}`);
            const clientHeight = minimap.clientHeight;
            console.log(`content height is ${clientHeight}`);
            const scrollHeight = document.documentElement.scrollHeight;
            console.log(`scroll height is ${scrollHeight}`);

            minimap.width = minimap.offsetWidth;
            minimap.height = minimap.offsetHeight;

            const scaleRatio = minimap.clientWidth / content.clientWidth;

            // draw background
            context.fillStyle = "#FFFFFF";
            context.fillRect(0, 0, minimap.width, minimap.height);

            const scaleRect = (rect: DOMRect, ratio: number): DOMRect => {
                return new DOMRect(
                    rect.x * ratio,
                    rect.y * ratio,
                    rect.width * ratio,
                    rect.height * ratio
                );
            };

            const range = document.createRange();

            const drawElement = (element: Element) => {
                if (element.classList.contains("minimap-exclude")) return;

                context.fillStyle = window
                    .getComputedStyle(element)
                    .getPropertyValue("background-color");
                const bgRect = scaleRect(
                    element.getBoundingClientRect(),
                    scaleRatio
                );
                context.fillRect(
                    bgRect.x,
                    bgRect.y,
                    bgRect.width,
                    bgRect.height
                );

                if (element.classList.contains("minimap-exclude-text")) return;

                const textColor = window
                    .getComputedStyle(element)
                    .getPropertyValue("color");

                const parts = textColor.match(/[\d.]+/g);
                // set alpha
                if (parts.length === 3) {
                    parts.push("0.3");
                } else {
                    parts[3] = "0.3";
                }
                context.fillStyle = `rgba(${parts.join(",")})`;

                for (const clientRect of element.getClientRects()) {
                    const rect = scaleRect(clientRect, scaleRatio);
                    context.fillRect(rect.x, rect.y, rect.width, rect.height);
                }
            };

            let root = content;
            const queue = [];
            while (root) {
                // console.log(root);
                drawElement(root);
                [...root.children].forEach((child) => queue.push(child));
                root = queue.shift();
            }

            // highlight scrollrect
            const scrollRect = new DOMRect(
                0,
                (scrollY * minimap.clientHeight) /
                    document.documentElement.scrollHeight,
                minimap.width,
                scaleRatio * clientHeight
            );
            if (scrollRect.height < scrollHeight * 0.8) {
                context.fillStyle = "rgba(255, 255, 255, 0.25)";
                context.fillRect(
                    scrollRect.x,
                    scrollRect.y,
                    scrollRect.width,
                    scrollRect.height
                );
            }
        });
    }

    onMount(() => {
        context = minimap.getContext("2d", { alpha: false });
        let resizeObserver = new ResizeObserver(repaint);
        resizeObserver.observe(content);

        repaint();
    });
</script>

<svelte:window bind:scrollY on:resize="{repaint}" />
<div class="content minimap-exclude" bind:this="{content}">
    <slot />
</div>
<canvas class="minimap" bind:this="{minimap}"></canvas>

<style>
    .content {
        margin-right: 10vw;
    }
    .minimap {
        position: fixed;
        top: 0;
        right: 0;
        min-width: 40px;
        max-width: 150px;
        width: 10vw;
        height: 100%;
        z-index: 100;
    }
</style>
