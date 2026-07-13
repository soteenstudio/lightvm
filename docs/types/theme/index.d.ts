import './style.css';
import { EnhanceAppContext } from 'vitepress';
declare const _default: {
    extends: {
        Layout: import("vue").DefineComponent;
        enhanceApp: (ctx: EnhanceAppContext) => void;
    };
    enhanceApp({ app }: EnhanceAppContext): void;
    Layout(): import("vue").VNode<import("vue").RendererNode, import("vue").RendererElement, {
        [key: string]: any;
    }>;
};
export default _default;
