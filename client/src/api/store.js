//门店管理api
import {post, get, put, del} from '@/utils/request'

export default {
  get:() => get('/store_url/store/stores'),//查询所有门店信息
  specific: storeName => get('/store_url/store/name/'+storeName+'/store'),//查询特定门店信息
  add: query => post('/store_url/store/store',query),//新增门店
  edit: (id,query) => put('/store_url/store/'+id+'/store',query),//修改门店信息
  del: id => del('/store_url/store/'+id+'/store')//删除门店信息
}
