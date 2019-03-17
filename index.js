const {
    getDriftForEachVariable,
    createInitialInputMetrics,
    updateInputMetrics,
    getKeys,
    generateConfusionMatrix
}=require('./js/metrics')
const fastify = require('fastify')({
    logger: true
})
const HISTORY_TO_KEEP=100
let inputData
let inputMetrics

const makeArrayIfNotArray=objOrArray=>Array.isArray(objOrArray)?objOrArray:[objOrArray]
// Declare a route
fastify.get('/', (request, reply) => {
    reply.send({ hello: 'world' })
})

fastify.post('/input', (request, reply)=>{
    const {inputValues, originalValues}=request.body
    const values=makeArrayIfNotArray(inputValues)
    if(inputData){
        const keys=getKeys(values)
        inputData=updateInputMetrics(keys, inputData, values, HISTORY_TO_KEEP)
    }
    else {
        inputData=createInitialInputMetrics(values, HISTORY_TO_KEEP)
    }
    inputMetrics=getDriftForEachVariable(inputData, originalValues)
    reply.send({success:true})
})


// Run the server!
fastify.listen(3000, (err, address) => {
    if (err) throw err
    fastify.log.info(`server listening on ${address}`)
})