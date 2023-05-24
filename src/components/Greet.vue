<template>

  <el-form :model="formInline" class="demo-form-inline" label-width="auto">
    <el-row>
      <el-form-item>
        <el-button type="primary" @click="dialogFormVisible = true">新增服务</el-button>
      </el-form-item>
    </el-row>
  </el-form>

  <el-table :data="tableData" style="width: 100%">
    <el-table-column prop="serverName" label="服务名称" width="100px"/>
    <el-table-column prop="serverPath" label="服务路径" />
    <el-table-column prop="serverCmd" label="启动命令"/>
    <el-table-column fixed="right" label="操作" width="220">
      <template #default="scope">
        <el-button type="primary" size="small" @click.prevent="starServer(scope.row)">启动</el-button>

        <el-popconfirm width="220" title="Are you sure to delete this server?" @confirm="deleteRow(scope.row)">
          <template #reference>
            <el-button type="danger" size="small">删除</el-button>
          </template>
        </el-popconfirm>
      </template>
    </el-table-column>
  </el-table>

  <el-dialog v-model="dialogFormVisible" v-show="dialogFormVisible" title="服务添加" center>
    <el-form ref="formRef" :model="form" :rules="rules">
      <el-form-item label="服务名称" :label-width="formLabelWidth" prop="serverName">
        <el-input v-model="form.serverName" autocomplete="off"/>
      </el-form-item>

      <el-form-item label="服务路径" :label-width="formLabelWidth" prop="serverPath">
        <el-input v-model="form.serverPath" autocomplete="off"/>
      </el-form-item>

      <el-form-item label="服务启动命令" :label-width="formLabelWidth" prop="serverCmd">
        <el-input v-model="form.serverCmd" autocomplete="off"/>
      </el-form-item>
    </el-form>
    <template #footer>
      <span class="dialog-footer">
        <el-button @click="resetForm(formRef)">重置</el-button>
        <el-button type="primary" @click="submitForm(formRef)">
          保存
        </el-button>
      </span>
    </template>
  </el-dialog>

</template>
<script lang="ts" setup>
import {onMounted, reactive, ref, toRaw} from 'vue'

import {invoke} from '@tauri-apps/api'
import {ElMessage, FormInstance, FormRules} from "element-plus";

const formLabelWidth = '140px'
const dialogFormVisible = ref(false)
const formRef = ref<FormInstance>()

const form = reactive({
  serverName: '',
  serverPath: '',
  serverCmd: '',
})

const rules = reactive<FormRules>({
  serverName: [{required: true, message: '请输入服务名称', trigger: 'blur'}],
  serverPath: [{required: true, message: '请输入安装路径', trigger: 'blur'}],
  serverCmd: [{required: true, message: '请输入启动路径', trigger: 'blur'}],
})

const tableData = ref([])

onMounted(() => {
  reloadTable()
})

const reloadTable = () => {
  let serverDataStr = localStorage.getItem('server') || '[]'
  tableData.value = JSON.parse(serverDataStr)
}

const deleteRow = (item) => {
  let serverDataStr = localStorage.getItem('server') || '[]'
  let serverList = JSON.parse(serverDataStr)

  let tmpList = serverList.filter(x => x.serverName != item.serverName)
  localStorage.setItem('server', JSON.stringify(tmpList));
  reloadTable()
}

const starServer = async (row) => {
  try {

    let serverPath = row.serverPath;
    let serverCmd = row.serverCmd;

    console.log('serverPath', serverPath)

    let split = serverCmd.split(" ");

    const cmdStr = ["/C" , ...split]

    console.log('cmdStr', cmdStr)

    const response = await invoke('start_server', {'path': serverPath,'cmdStr': cmdStr});

    console.log('starServer response', response)
    if (response.err) {
      console.error(response.err)
    } else {
      console.log(response.result)
    }
  } catch (error) {
    console.error(error); // "Failed to start Minio!"
  }

}

const submitForm = async (formEl: FormInstance | undefined) => {
  if (!formEl) return

  console.log('form', formEl)

  await formEl.validate((valid, fields) => {
    if (valid) {
      console.log('submit!')

      let serverDataStr = localStorage.getItem('server') || '[]'
      let serverList = JSON.parse(serverDataStr)
      console.log('serverList', serverList)

      if (serverList.some(x => x.serverName == form.serverName)) {
        ElMessage({
          message: `${form.serverName}已存在`,
          type: 'warning',
        })
        return
      }
      serverList.push(toRaw(form))
      localStorage.setItem('server', JSON.stringify(serverList));
      dialogFormVisible.value = false
      reloadTable()
    } else {
      console.log('error submit!', fields)
    }
  })
}

const resetForm = (formEl: FormInstance | undefined) => {
  if (!formEl) return
  formEl.resetFields()
}


const formInline = reactive({
  idCard: '',
  name: '',
})


const updateServer = (row) => {
  try {

  } catch (error) {
    console.error(error);
  }
}


</script>
<style scoped>
.demo-tabs > .el-tabs__content {
  padding: 32px;
  color: #6b778c;
  font-size: 32px;
  font-weight: 600;
}
</style>
