import axios from 'axios'
import vue from 'vue'

const request = function (loadtip, query) {
  let loading // 页面加载部分设置
  if (loadtip) {
    loading = vue.prototype.$loading({
      lock: false,
      text: '正在加载中…',
      spinner: 'el-icon-loading',
      background: 'rgba(0, 0, 0, 0.5)'
    })
  }
  // 在请求响应的地方回设token
  return axios.request(query)
    .then(res => {
      if (loadtip) {
        // 取消页面加载tip
        loading.close()}
      // 登陆失败，请求异常信息提示处理
      if (res.data.code === 401) {
        vue.prototype.$router.push({ path: '/login' })
        return Promise.reject(res.data)
      }
      else if (res.data.code === 500) {
        return Promise.reject(res.data)
      } else if (res.data.code === 501) {
        return Promise.reject(res.data)
      } else if (res.data.code === 502) {
        vue.prototype.$router.push({ path: '/login' })
        return Promise.reject(res.data)}
      //登陆成功
      else {
        // 设置token
        let token = res.headers['token']
        if (token && token !== '') {
          localStorage.setItem('token', token)
        }
        //设置tokenend
        return Promise.resolve(res.data)
      }
    })
    .catch(e => {
      if (loadtip) { loading.close() }
      vue.prototype.$message.error(e.message)
      return Promise.reject(e.message)
    })
}

// 请求方法封装
/*
 *  post请求
 *  url:请求地址
 *  params:参数
 * */

const post = function (url, params) {
  const query = {
    url: url,
    method: 'post',
    withCredentials: true,
    timeout: 30000,
    data: params,
    headers: { 'Content-Type': 'application/json', 'request-ajax': true }
  }
  const token = localStorage.getItem('token')
  if (token) {
    query.headers.token = token
  }
  return request(false, query)
}

/*
 *  get请求
 *  url:请求地址
 *  params:参数
 * */

const get = function (url, params) {
  const query = {
    url: url,
    method: 'get',
    withCredentials: true,
    timeout: 30000,
    params: params,
    headers: { 'request-ajax': true }
  }
  const token = localStorage.getItem('token')
  if (token) {
    query.headers.token = token
  }
  return request(false, query)
}

const put = function (url, params) {
  const query = {
    url: url,
    method: 'put',
    withCredentials: true,
    timeout: 30000,
    data: params
  }
  const token = localStorage.getItem('token')
  if (token) {
    query.headers.token = token
  }
  return request(false, query)
}

const del = function (url, params) {
  const query = {
    url: url,
    method: 'delete',
    withCredentials: true,
    timeout: 30000,
    data: params,
    headers: { 'Content-Type': 'application/json', 'request-ajax': true }
  }
  const token = localStorage.getItem('token')
  if (token) {
    query.headers.token = token
  }
  return request(false, query)
}

export {
  post,
  get,
  put,
  del
}
