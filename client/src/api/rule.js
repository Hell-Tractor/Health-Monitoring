//排班规则api
import {get,put} from '@/utils/request'
export default {
  get:()=>get('/rule_url/rule/rules'),//查询所有排班规则
  specific: storeId => get('/rule_url'+storeId+'/rules'),//查询特定门店的排班规则
  getByType:(storeId,type)=>get('/rule_url/rule/'+storeId+'/'+type+'/rule'),//查询特定门店特定类别的排班规则
  edit:(storeId,type,query)=>put('/rule_url/rule/'+storeId+'/'+type+'/rule', query)//修改特定门店特定类别的排班规则
}
