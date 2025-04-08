import DashboardView from "./dashboard/index.vue";
import WidgetView from "./widget/index.vue";
import ErrorView from "./error/index.vue";

export type AppView = keyof typeof views;
export const views = {
  main: DashboardView,
  widget: WidgetView,
  error: ErrorView
};

export { WidgetView, DashboardView, ErrorView };
