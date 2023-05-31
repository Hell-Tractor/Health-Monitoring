module.exports = {
  lintOnSave: false,
  devServer: {
    open: true,
    host: 'localhost', // 后端接口的域名
    port: 8080, // 前端启动的端口
    https: false,
    hotOnly: false,
    // 接口名前面要有api
    proxy: {
      '/store_url': {
        //target: 'http://192.168.202.1:8081', // 后端接口地址
        target: 'http://localhost:8081',
        changeOrigin: true,
        pathRewrite: {
          "^/store_url": ''
        }
      },
      '/staff_url': {
        //target: 'http://192.168.202.1:8082', // 后端接口地址
        target: 'http://localhost:8082',
        changeOrigin: true,
        pathRewrite: {
          "^/staff_url": ''
        }
      },
      '/rule_url': {
        //target: 'http://192.168.202.1:8083', // 后端接口地址
        target: 'http://localhost:8083',
        changeOrigin: true,
        pathRewrite: {
          "^/rule_url": ''
        }
      },
      '/forecast_url': {
        //target: 'http://192.168.202.1:8084', // 后端接口地址
        target: 'http://localhost:8084',
        changeOrigin: true,
        pathRewrite: {
          "^/forecast_url": ''
        }
      },
      '/preference_url':{
        //target: 'http://192.168.202.1:8082',
        target: 'http://localhost:8082',
        changeOrigin: true,
        pathRewrite: {
          "^/preference_url": ''
        }
      },
      '/schedule_url':{
        //target: 'http://192.168.202.1:8085',
        target: 'http://localhost:8085',
        changeOrigin: true,
        pathRewrite: {
          "^/schedule_url": ''
        }
      },
    }
  },
};
