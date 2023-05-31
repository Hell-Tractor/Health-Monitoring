import HomeLayout from "@/layout/HomeLayout.vue";
import NotFound from "@/views/NotFoundPage.vue";
import Home from "@/views/Home.vue";
import StaticsManagement from "@/views/Statics.vue";

const routes = [
  {
    path: "/",
    component: HomeLayout,
    redirect: "/home",
    children: [
      {
        path: "/home",
        name: "home",
        component: Home,
      },
      {
        path: "/statics",
        name: "statics",
        component: StaticsManagement,
      }
    ],
  },
  { path: "*", component: NotFound },
];

export default routes;
