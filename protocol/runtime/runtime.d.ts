declare module "sown-runtime:console" {
    export interface Console {
        log(...data: any[]): void;
    }
}

declare var console: import("sown-runtime:console").Console
