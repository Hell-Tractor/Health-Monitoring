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
      '/data_url': {
        target: 'http://120.25.236.100:9999', // 后端接口地址
        changeOrigin: true,
        pathRewrite: {
          "^/data_url": ''
        }
      }
    }
  },
};
