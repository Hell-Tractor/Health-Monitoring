<template>
  <div>
    <el-row :gutter="20">
      <el-col :span="7" style="margin-left: 40px;">
        <stats-card>
          <div class="icon-big text-center" :class="`icon-danger`" slot="header">
            <i class="ti-heart"></i>
          </div>
          <div class="numbers" slot="content">
            <p>心率</p>{{this.heartRate}}
          </div>
          <div class="stats" slot="footer">
            <i class="ti-alert"  :class="`icon-warning`"></i> {{ this.heartRateTime}}  {{ this.heartRateValue }} 次/分
          </div>
        </stats-card>
      </el-col>
      <el-col :span="7" style="margin-left: 40px;">
        <stats-card>
          <div class="icon-big text-center" :class="`icon-success`" slot="header">
            <i class="ti-image"></i>
          </div>
          <div class="numbers" slot="content">
            <p>血压</p>{{this.bloodPressureLow}} / {{this.bloodPressureHigh}}
          </div>
          <div class="stats" slot="footer">
            <i class="ti-alert" :class="`icon-warning`"></i> {{ this.bloodPressureTime}}  高压 {{ this.bloodPressureValue }} 
          </div>
        </stats-card>
      </el-col>
      <el-col :span="7" style="margin-left: 40px;">
        <stats-card>
          <div class="icon-big text-center" :class="`icon-info`" slot="header">
            <i class="ti-pulse"></i>
          </div>
          <div class="numbers" slot="content">
            <p>血氧饱和度</p>{{this.bloodOxygen}}%
          </div>
          <div class="stats" slot="footer">
            <i class="ti-alert" :class="`icon-warning`"></i> {{ this.bloodOxygenTime}} {{ this.bloodOxygenValue }}%
          </div>
        </stats-card>
      </el-col>
    </el-row>
    <el-row :gutter="20" style="margin-top: 10px;">
      <el-col :span="11" style="margin-left: 40px;">
        <div class="card">
          <div class="card-body">
            <div id="echarts" style="width: 500px; height: 305px; margin-top: 25px;"></div>
          </div>
        </div>
      </el-col>
      <el-col style="width: 578px; margin-left: 10px">
        <div class="card">
          <div class="card-body">
            <div id="distance_chart" style="width: 500px; height: 325px; margin-top: 5px;"></div>
          </div>
        </div>
      </el-col>
    </el-row>
  </div>
  
</template>

<script>
import StatsCard from "@/components/Cards/StatsCard";
import dataApi from '@/api/data';
import * as echarts from 'echarts';

export default {
  components: {
    StatsCard,
  },
  data() {
    return {
      heartRate:'',
      bloodPressureLow:'',
      bloodPressureHigh:'',
      bloodOxygen:'',
      heartRateTime:'',
      heartRateValue:'',
      bloodPressureTime:'',
      bloodPressureValue:'',
      bloodOxygenTime:'',
      bloodOxygenValue:'',
      chart: null,
      distanceChart: null,
      floorNum:[],
      stepNum:[],
      walkDistance:[],
      runDistance:[]
    }
  },
  async created() {
    this.heartRate = await dataApi.getHeartRate().then(re => {
      return re.data[0].value
    })
    await dataApi.getHeartRateWarn().then(re => {
      this.heartRateTime=re.time
      this.heartRateValue=re.value
    })
    this.bloodPressureLow =await dataApi.getBloodPressureLow().then(re => {
      return re.data[0].value
    })
    this.bloodPressureHigh =await dataApi.getBloodPressureHigh().then(re => {
      return re.data[0].value
    })
    await dataApi.getBloodPressureWarn().then(re => {
        this.bloodPressureTime=re.time
        this.bloodPressureValue=re.value
    })
    this.bloodOxygen =await dataApi.getBloodOxygen().then(re => {
      return re.data[0].value
    })
    await dataApi.getBloodOxygenWarn().then(re => {
        this.bloodOxygenTime=re.time
        this.bloodOxygenValue=re.value
    })
    await dataApi.getFloorNumByPeriod().then(re => {
      var arr=[]
      for(var item in re){
        arr.push(re[item])
        console.log(re[item])
      }
      console.log(re.length)
      for(var i=0;i<6;i++) {
        this.floorNum.push(arr[i].value)
        console.log(arr[i].value)
      }
    })
    await dataApi.getStepNumByPeriod().then(re => {
      var arr=[]
      
      for(var item in re){
        arr.push(re[item])
      }
      console.log(re.length)
      for(var i=0;i<6;i++) {
        this.stepNum.push(arr[i].value)
      }
    })
    this.init1()
    await dataApi.getWalkDistanceByPeriod().then(re => {
      var arr=[]
      for(var item in re){
        arr.push(re[item])
      }
      console.log(re.length)
      for(var i=0;i<6;i++) {
        this.walkDistance.push(arr[i].value)
      }
    })
    await dataApi.getRunDistanceByPeriod().then(re => {
      var arr=[]
      for(var item in re){
        arr.push(re[item])
      }
      console.log(re.length)
      for(var i=0;i<6;i++) {
        this.runDistance.push(arr[i].value)
      }
    })
    this.init2()
  },
  methods: {
    async update() {
      this.heartRate = await dataApi.getHeartRate().then(re => {
        return re.data[0].value
      })
      this.bloodPressureLow =await dataApi.getBloodPressureLow().then(re => {
        return re.data[0].value
      })
      this.bloodPressureHigh =await dataApi.getBloodPressureHigh().then(re => {
        return re.data[0].value
      })
      this.bloodOxygen =await dataApi.getBloodOxygen().then(re => {
        return re.data[0].value
      })
      await dataApi.getFloorNumByPeriod().then(re => {
        var arr=[]
        for(var item in re){
          arr.push(re[item])
        }
        console.log(re.length)
        for(var i=0;i<6;i++) {
          this.floorNum[i]=(arr[i].value)
        }
      })
      await dataApi.getStepNumByPeriod().then(re => {
        var arr=[]
        for(var item in re){
          arr.push(re[item])
        }
        console.log(re.length)
        for(var i=0;i<6;i++) {
          this.stepNum[i]=(arr[i].value)
        }
      })
      this.init1()
      await dataApi.getWalkDistanceByPeriod().then(re => {
        var arr=[]
        for(var item in re){
          arr.push(re[item])
        }
        console.log(re.length)
        for(var i=0;i<6;i++) {
          this.walkDistance[i]=(arr[i].value)
        }
      })
      await dataApi.getRunDistanceByPeriod().then(re => {
        var arr=[]
        for(var item in re){
          arr.push(re[item])
        }
        console.log(re.length)
        for(var i=0;i<6;i++) {
          this.runDistance[i]=(arr[i].value)
        }
      })
      this.init2()
    },

    init1() {
      // 初始化
      this.chart = echarts.init(document.getElementById("echarts"));
      // 配置数据
      let option = {
        tooltip: {
          trigger: 'axis',
          axisPointer: {
            type: 'cross',
            crossStyle: {
              color: '#999'
            }
          }
        },
        toolbox: {
          feature: {
            //dataView: { show: true, readOnly: false },
            magicType: { show: true, type: ['line', 'bar'] },
            //restore: { show: true },
            //saveAsImage: { show: true }
          }
        },
        legend: {
          data: ['已爬楼层', '步数']
        },
        xAxis: [
          {
            type: 'category',
            data: ['0-4h', '4-8h', '8-12h', '12-16h', '16-20h', '20-24h'],
            axisPointer: {
              type: 'shadow'
            }
          }
        ],
        yAxis: [
          {
            type: 'value',
            name: '已爬楼层',
            min: 0,
            max: 25,
            interval: 5,
            axisLabel: {
              formatter: '{value}'
            }
          },
          {
            type: 'value',
            name: '步数',
            min: 0,
            max: 10000,
            interval: 2000,
            axisLabel: {
              formatter: '{value}'
            }
          }
        ],
        series: [
          {
            name: '已爬楼层',
            type: 'bar',
            tooltip: {
              valueFormatter: function (value) {
                return value;
              }
            },
            data: this.floorNum
          },
          {
            name: '步数',
            type: 'line',
            yAxisIndex: 1,
            tooltip: {
              valueFormatter: function (value) {
                return value;
              }
            },
            data: this.stepNum
          }
        ]
      };
      // 传入数据
      this.chart.setOption(option);
    },
    init2() {
      this.distanceChart = echarts.init(document.getElementById("distance_chart"));
      let option = {
        tooltip: {
          trigger: 'axis',
          axisPointer: {
            // Use axis to trigger tooltip
            type: 'shadow' // 'shadow' as default; can also be 'line' or 'shadow'
          }
        },
        legend: {},
        grid: {
          left: '3%',
          right: '4%',
          bottom: '3%',
          containLabel: true
        },
        xAxis: {
          type: 'value'
        },
        yAxis: {
          type: 'category',
          data: ['0-4h', '4-8h', '8-12h', '12-16h', '16-20h', '20-24h']
        },
        series: [
          {
            name: '步行距离',
            type: 'bar',
            stack: 'total',
            label: {
              show: true
            },
            emphasis: {
              focus: 'series'
            },
            data: this.walkDistance
          },
          {
            name: '跑步距离',
            type: 'bar',
            stack: 'total',
            label: {
              show: true
            },
            emphasis: {
              focus: 'series'
            },
            data: this.runDistance
          }
        ]
      };
      this.distanceChart.setOption(option);
    }
  },
  mounted() {
    console.log(this.floorNum),
    this.init1(),
    this.init2(),
    this.update()
  }
}
</script>

<style lang="scss" scoped>
</style>
