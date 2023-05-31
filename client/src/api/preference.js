//员工偏好api
import {get,put} from '@/utils/request'
export default {
  get: ()=>get('/preference_url/staff/preferences'),//查询所有员工偏好
  getByType: type=>get('/preference_url/staff/preferences/'+type+'/preferences'),//查询某方面偏好
  edit: (staffId,preferenceId,type,query)=>put('/preference_url/staff/'+staffId+'/preferences/'+preferenceId+'/preference/'+type,query),//修改员工偏好
  getId:(staffId,type)=>get('/preference_url/staff/'+staffId+'/preferences/'+type+'/preference/id')//获得偏好id
}
