import Vue from "vue";
import VueRouter from "vue-router";
import routes from "./routes";
Vue.use(VueRouter);

const router = new VueRouter({
  mode: 'hash',
  routes,
  linkActiveClass: "active",
});

export default router;
