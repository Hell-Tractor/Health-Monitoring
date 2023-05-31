//员工管理api
import {post, get, put, del} from '@/utils/request'

export default {
  get:() => get('/staff_url/staff/staffs'),//查询所有员工信息
  specific: staffName => get('/staff_url/staff/name/'+staffName+'/staff'),//根据名字查询特定员工信息
  store_staff: storeId => get('/staff_url/staff'+storeId+'/staffs'), //获取门店员工
  add: query => post('/staff_url/staff/staff',query),//新增员工
  edit: (id,query) => put('/staff_url/staff/'+id+'/staff',query),//修改员工信息
  del: id => del('/staff_url/staff/'+id+'/staff'),//删除员工信息
  getId: staffName => get('/staff_url/staff/name/'+staffName+"/staffId"),//获得员工id
}
