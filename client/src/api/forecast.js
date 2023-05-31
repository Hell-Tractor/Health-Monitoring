//业务预测api
import {get} from '@/utils/request'
export default {
  getByStore:storeId=>get('/forecast_url/forecast/'+storeId+'/data'),//查看特定门店的所有业务数据
  getByPeriod:storeId=>get('/forecast_url/forecast/'+storeId+'/period'),//查看特定门店特定时间段的业务预测数据
  getByDay:(storeId,date)=>get('/forecast_url/forecast/'+storeId+'/'+date+'/day'),//查看特定门店一周的业务预测数据
  getByTime:(storeId,date,time)=>get('/forecast_url/forecast/'+storeId+'/'+date+'/'+time+'/data')//查看特定门店特定日特定时间段的业务预测数据
}
