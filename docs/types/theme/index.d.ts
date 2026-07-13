import './style.css';
declare const _default: {
    extends: {
        Layout: import("vue").DefineComponent;
        enhanceApp: (ctx: import("vitepress").EnhanceAppContext) => void;
    };
    enhanceApp({ app }: {
        app: any;
    }): void;
    Layout(): import("vue").VNode<import("vue").RendererNode, import("vue").RendererElement, {
        [key: string]: any;
    }>;
};
export default _default;
